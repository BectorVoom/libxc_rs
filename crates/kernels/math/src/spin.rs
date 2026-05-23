//! Spin polarization transforms for DFT calculations.
//!
//! Provides functions for computing total density, spin polarization (zeta),
//! spin-scaling factor, and clamping zeta to valid range.
//! Generic over `<F: Float>` to support both f64 and f32.

use cubecl::prelude::*;
use super::powers::pow_4_3;

/// Compute total density from spin-up and spin-down densities.
///
/// Returns `rho_up + rho_down`.
#[cube]
pub fn compute_total<F: Float>(rho_up: F, rho_down: F) -> F {
    rho_up + rho_down
}

/// Compute spin polarization zeta = (rho_up - rho_down) / (rho_up + rho_down).
///
/// If total density is below `threshold`, returns 0.0 (unpolarized).
#[cube]
pub fn compute_zeta<F: Float>(rho_up: F, rho_down: F, threshold: F) -> F {
    let total = rho_up + rho_down;
    let zeta = (rho_up - rho_down) / total;
    select(total < threshold, F::new(0.0), zeta)
}

/// Combined total+zeta computation (convenience wrapper).
/// Returns total density. Use `compute_zeta` separately for zeta.
#[cube]
pub fn to_total_zeta_total<F: Float>(rho_up: F, rho_down: F) -> F {
    compute_total::<F>(rho_up, rho_down)
}

/// Spin scaling factor: f(zeta) = ((1+zeta)^(4/3) + (1-zeta)^(4/3)) / 2.
///
/// Approaches 1.0 for unpolarized (zeta=0) and 2^(1/3) for fully polarized (zeta=1).
#[cube]
pub fn spin_scaling<F: Float>(zeta: F) -> F {
    let up = F::new(1.0) + zeta;
    let down = F::new(1.0) - zeta;
    (pow_4_3::<F>(up) + pow_4_3::<F>(down)) / F::new(2.0)
}

/// Clamp zeta to [-(1-threshold), (1-threshold)].
///
/// Prevents division by zero in spin-dependent quantities when
/// one spin channel has nearly zero density.
#[cube]
pub fn clamp_zeta<F: Float>(zeta: F, threshold: F) -> F {
    let upper = F::new(1.0) - threshold;
    let lower = -(F::new(1.0) - threshold);
    let clamped = select(zeta > upper, upper, zeta);
    select(clamped < lower, lower, clamped)
}

#[cfg(test)]
mod tests {
    use super::*;
    use cubecl::cpu::{CpuDevice, CpuRuntime};
    use cubecl::Runtime;
    use cubecl::client::ComputeClient;

    // Test kernel for compute_total + compute_zeta (outputs 2 values per input pair)
    #[cube(launch_unchecked)]
    fn test_total_zeta_kernel(
        rho_up: &Array<f64>,
        rho_down: &Array<f64>,
        threshold: &Array<f64>,
        out_total: &mut Array<f64>,
        out_zeta: &mut Array<f64>,
    ) {
        let idx = ABSOLUTE_POS;
        out_total[idx] = compute_total::<f64>(rho_up[idx], rho_down[idx]);
        out_zeta[idx] = compute_zeta::<f64>(rho_up[idx], rho_down[idx], threshold[idx]);
    }

    #[cube(launch_unchecked)]
    fn test_spin_scaling_kernel(input: &Array<f64>, output: &mut Array<f64>) {
        let idx = ABSOLUTE_POS;
        output[idx] = spin_scaling::<f64>(input[idx]);
    }

    #[cube(launch_unchecked)]
    fn test_clamp_zeta_kernel(
        zeta: &Array<f64>,
        threshold: &Array<f64>,
        output: &mut Array<f64>,
    ) {
        let idx = ABSOLUTE_POS;
        output[idx] = clamp_zeta::<f64>(zeta[idx], threshold[idx]);
    }

    fn make_client() -> ComputeClient<CpuRuntime> {
        let device = CpuDevice;
        CpuRuntime::client(&device)
    }

    fn run_total_zeta(rho_up: &[f64], rho_down: &[f64], threshold: &[f64]) -> (Vec<f64>, Vec<f64>) {
        let client = make_client();
        let n = rho_up.len();
        let sz = core::mem::size_of::<f64>();

        let h_up = client.create_from_slice(bytemuck::cast_slice(rho_up));
        let h_down = client.create_from_slice(bytemuck::cast_slice(rho_down));
        let h_thr = client.create_from_slice(bytemuck::cast_slice(threshold));
        let h_total = client.empty(n * sz);
        let h_zeta = client.empty(n * sz);

        unsafe {
            test_total_zeta_kernel::launch_unchecked::<CpuRuntime>(
                &client,
                CubeCount::new_1d(n as u32),
                CubeDim::new_1d(1),
                ArrayArg::from_raw_parts(h_up, n),
                ArrayArg::from_raw_parts(h_down, n),
                ArrayArg::from_raw_parts(h_thr, n),
                ArrayArg::from_raw_parts(h_total.clone(), n),
                ArrayArg::from_raw_parts(h_zeta.clone(), n),
            );
        }

        let total_bytes = client.read_one(h_total).expect("read_one failed during output buffer read-back");
        let zeta_bytes = client.read_one(h_zeta).expect("read_one failed during output buffer read-back");
        (
            bytemuck::cast_slice(&total_bytes).to_vec(),
            bytemuck::cast_slice(&zeta_bytes).to_vec(),
        )
    }

    fn run_spin_scaling(values: &[f64]) -> Vec<f64> {
        let client = make_client();
        let n = values.len();
        let input_handle = client.create_from_slice(bytemuck::cast_slice(values));
        let output_handle = client.empty(n * core::mem::size_of::<f64>());

        unsafe {
            test_spin_scaling_kernel::launch_unchecked::<CpuRuntime>(
                &client,
                CubeCount::new_1d(n as u32),
                CubeDim::new_1d(1),
                ArrayArg::from_raw_parts(input_handle, n),
                ArrayArg::from_raw_parts(output_handle.clone(), n),
            );
        }

        let bytes = client.read_one(output_handle).expect("read_one failed during output buffer read-back");
        bytemuck::cast_slice(&bytes).to_vec()
    }

    fn run_clamp_zeta(zeta: &[f64], threshold: &[f64]) -> Vec<f64> {
        let client = make_client();
        let n = zeta.len();
        let h_zeta = client.create_from_slice(bytemuck::cast_slice(zeta));
        let h_thr = client.create_from_slice(bytemuck::cast_slice(threshold));
        let h_out = client.empty(n * core::mem::size_of::<f64>());

        unsafe {
            test_clamp_zeta_kernel::launch_unchecked::<CpuRuntime>(
                &client,
                CubeCount::new_1d(n as u32),
                CubeDim::new_1d(1),
                ArrayArg::from_raw_parts(h_zeta, n),
                ArrayArg::from_raw_parts(h_thr, n),
                ArrayArg::from_raw_parts(h_out.clone(), n),
            );
        }

        let bytes = client.read_one(h_out).expect("read_one failed during output buffer read-back");
        bytemuck::cast_slice(&bytes).to_vec()
    }

    #[test]
    fn test_to_total_zeta_polarized() {
        let (totals, zetas) = run_total_zeta(&[0.5], &[0.3], &[1e-15]);
        approx::assert_relative_eq!(totals[0], 0.8, max_relative = 1e-15);
        approx::assert_relative_eq!(zetas[0], 0.25, max_relative = 1e-15);
    }

    #[test]
    fn test_to_total_zeta_unpolarized() {
        let (totals, zetas) = run_total_zeta(&[0.5], &[0.5], &[1e-15]);
        approx::assert_relative_eq!(totals[0], 1.0, max_relative = 1e-15);
        assert_eq!(zetas[0], 0.0);
    }

    #[test]
    fn test_to_total_zeta_below_threshold() {
        let (totals, zetas) = run_total_zeta(&[1e-20], &[1e-20], &[1e-15]);
        approx::assert_relative_eq!(totals[0], 2e-20, max_relative = 1e-14);
        assert_eq!(zetas[0], 0.0); // below threshold -> zeta=0
    }

    #[test]
    fn test_clamp_zeta_within_range() {
        let results = run_clamp_zeta(&[0.5], &[1e-10]);
        assert_eq!(results[0], 0.5);
    }

    #[test]
    fn test_clamp_zeta_positive_overflow() {
        let results = run_clamp_zeta(&[1.5], &[1e-10]);
        approx::assert_relative_eq!(results[0], 1.0 - 1e-10, max_relative = 1e-14);
    }

    #[test]
    fn test_clamp_zeta_negative_overflow() {
        let results = run_clamp_zeta(&[-1.5], &[1e-10]);
        approx::assert_relative_eq!(results[0], -(1.0 - 1e-10), max_relative = 1e-14);
    }

    #[test]
    fn test_spin_scaling_unpolarized() {
        // f(0) = ((1)^(4/3) + (1)^(4/3)) / 2 = 1.0
        let results = run_spin_scaling(&[0.0]);
        approx::assert_relative_eq!(results[0], 1.0, max_relative = 1e-14);
    }

    #[test]
    fn test_spin_scaling_fully_polarized() {
        // f(1) = ((2)^(4/3) + (0)^(4/3)) / 2 = 2^(4/3) / 2 = 2^(1/3)
        let results = run_spin_scaling(&[1.0]);
        let expected = 2.0_f64.powf(1.0 / 3.0);
        approx::assert_relative_eq!(results[0], expected, max_relative = 1e-14);
    }
}
