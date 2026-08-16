//! DFT-specific derived quantities.
//!
//! Wigner-Seitz radius, reduced gradient, Thomas-Fermi kinetic energy density,
//! and dimensionless inhomogeneity parameter alpha.

use cubecl::prelude::*;
use super::constants::{RS_CONST, KF_CONST};
use super::powers::{pow_1_3, pow_4_3, pow_5_3};

/// Wigner-Seitz radius: rs = RS_CONST * rho^(-1/3) = RS_CONST / cbrt(rho).
///
/// RS_CONST = (3/(4*pi))^(1/3)
#[cube]
pub fn wigner_seitz_rs<F: Float>(rho: F) -> F {
    F::cast_from(RS_CONST) * pow_1_3::<F>(F::cast_from(1.0_f64) / rho)
}

/// Reduced density gradient: s = sqrt(sigma) / (2 * kf * rho^(4/3))
///
/// where kf = KF_CONST * rho^(1/3) so 2*kf*rho^(4/3) = 2*KF_CONST*rho^(1/3)*rho^(4/3) = 2*KF_CONST*rho^(5/3)
/// Actually, the standard form is: s = |grad rho| / (2 * kF * rho) where kF = (3*pi^2*rho)^(1/3)
/// So s = sqrt(sigma) / (2 * KF_CONST * rho^(4/3))
#[cube]
pub fn reduced_gradient_s<F: Float>(rho: F, sigma: F) -> F {
    F::sqrt(sigma) / (F::cast_from(2.0_f64) * F::cast_from(KF_CONST) * pow_4_3::<F>(rho))
}

/// Thomas-Fermi kinetic energy density: t_TF = (3/10) * (3*pi^2)^(2/3) * rho^(5/3)
///
/// Note: (3*pi^2)^(2/3) = KF_CONST^2
#[cube]
pub fn tf_kinetic<F: Float>(rho: F) -> F {
    F::cast_from(0.3_f64) * F::cast_from(KF_CONST) * F::cast_from(KF_CONST) * pow_5_3::<F>(rho)
}

/// Dimensionless inhomogeneity parameter alpha:
/// alpha = (tau - tau_W) / tau_TF
///
/// where tau_W = sigma/(8*rho) is the von Weizsacker kinetic energy density
/// and tau_TF is the Thomas-Fermi kinetic energy density.
///
/// alpha = 1 for the uniform electron gas (tau = tau_TF, sigma = 0).
#[cube]
pub fn dimensionless_alpha<F: Float>(rho: F, sigma: F, tau: F) -> F {
    let tau_w = sigma / (F::cast_from(8.0_f64) * rho);
    let tau_tf = tf_kinetic::<F>(rho);
    (tau - tau_w) / tau_tf
}

#[cfg(test)]
mod tests {
    use super::*;
    use cubecl::cpu::{CpuDevice, CpuRuntime};
    use cubecl::Runtime;
    use cubecl::client::ComputeClient;

    #[cube(launch_unchecked)]
    fn test_rs_kernel(rho: &Array<f64>, output: &mut Array<f64>) {
        let idx = ABSOLUTE_POS;
        output[idx] = wigner_seitz_rs::<f64>(rho[idx]);
    }

    #[cube(launch_unchecked)]
    fn test_reduced_gradient_kernel(
        rho: &Array<f64>,
        sigma: &Array<f64>,
        output: &mut Array<f64>,
    ) {
        let idx = ABSOLUTE_POS;
        output[idx] = reduced_gradient_s::<f64>(rho[idx], sigma[idx]);
    }

    #[cube(launch_unchecked)]
    fn test_tf_kinetic_kernel(rho: &Array<f64>, output: &mut Array<f64>) {
        let idx = ABSOLUTE_POS;
        output[idx] = tf_kinetic::<f64>(rho[idx]);
    }

    #[cube(launch_unchecked)]
    fn test_alpha_kernel(
        rho: &Array<f64>,
        sigma: &Array<f64>,
        tau: &Array<f64>,
        output: &mut Array<f64>,
    ) {
        let idx = ABSOLUTE_POS;
        output[idx] = dimensionless_alpha::<f64>(rho[idx], sigma[idx], tau[idx]);
    }

    fn make_client() -> ComputeClient<CpuRuntime> {
        let device = CpuDevice;
        CpuRuntime::client(&device)
    }

    fn run_rs(rho: &[f64]) -> Vec<f64> {
        let client = make_client();
        let n = rho.len();
        let h_rho = client.create_from_slice(bytemuck::cast_slice(rho));
        let h_out = client.empty(n * core::mem::size_of::<f64>());

        unsafe {
            test_rs_kernel::launch_unchecked::<CpuRuntime>(
                &client,
                CubeCount::new_1d(n as u32),
                CubeDim::new_1d(1),
                ArrayArg::from_raw_parts(h_rho, n),
                ArrayArg::from_raw_parts(h_out.clone(), n),
            );
        }

        let bytes = client.read_one(h_out).expect("read_one failed during output buffer read-back");
        bytemuck::cast_slice(&bytes).to_vec()
    }

    fn run_tf(rho: &[f64]) -> Vec<f64> {
        let client = make_client();
        let n = rho.len();
        let h_rho = client.create_from_slice(bytemuck::cast_slice(rho));
        let h_out = client.empty(n * core::mem::size_of::<f64>());

        unsafe {
            test_tf_kinetic_kernel::launch_unchecked::<CpuRuntime>(
                &client,
                CubeCount::new_1d(n as u32),
                CubeDim::new_1d(1),
                ArrayArg::from_raw_parts(h_rho, n),
                ArrayArg::from_raw_parts(h_out.clone(), n),
            );
        }

        let bytes = client.read_one(h_out).expect("read_one failed during output buffer read-back");
        bytemuck::cast_slice(&bytes).to_vec()
    }

    fn run_alpha(rho: &[f64], sigma: &[f64], tau: &[f64]) -> Vec<f64> {
        let client = make_client();
        let n = rho.len();
        let sz = core::mem::size_of::<f64>();
        let h_rho = client.create_from_slice(bytemuck::cast_slice(rho));
        let h_sig = client.create_from_slice(bytemuck::cast_slice(sigma));
        let h_tau = client.create_from_slice(bytemuck::cast_slice(tau));
        let h_out = client.empty(n * sz);

        unsafe {
            test_alpha_kernel::launch_unchecked::<CpuRuntime>(
                &client,
                CubeCount::new_1d(n as u32),
                CubeDim::new_1d(1),
                ArrayArg::from_raw_parts(h_rho, n),
                ArrayArg::from_raw_parts(h_sig, n),
                ArrayArg::from_raw_parts(h_tau, n),
                ArrayArg::from_raw_parts(h_out.clone(), n),
            );
        }

        let bytes = client.read_one(h_out).expect("read_one failed during output buffer read-back");
        bytemuck::cast_slice(&bytes).to_vec()
    }

    fn run_reduced_gradient(rho: &[f64], sigma: &[f64]) -> Vec<f64> {
        let client = make_client();
        let n = rho.len();
        let sz = core::mem::size_of::<f64>();
        let h_rho = client.create_from_slice(bytemuck::cast_slice(rho));
        let h_sig = client.create_from_slice(bytemuck::cast_slice(sigma));
        let h_out = client.empty(n * sz);

        unsafe {
            test_reduced_gradient_kernel::launch_unchecked::<CpuRuntime>(
                &client,
                CubeCount::new_1d(n as u32),
                CubeDim::new_1d(1),
                ArrayArg::from_raw_parts(h_rho, n),
                ArrayArg::from_raw_parts(h_sig, n),
                ArrayArg::from_raw_parts(h_out.clone(), n),
            );
        }

        let bytes = client.read_one(h_out).expect("read_one failed during output buffer read-back");
        bytemuck::cast_slice(&bytes).to_vec()
    }

    #[test]
    fn test_wigner_seitz_rs_unit() {
        // For rho = 3/(4*pi), rs should be 1.0
        // rs = RS_CONST * (1/rho)^(1/3) = (3/(4pi))^(1/3) * (4pi/3)^(1/3) = 1.0
        let rho = 3.0 / (4.0 * std::f64::consts::PI);
        let results = run_rs(&[rho]);
        approx::assert_relative_eq!(results[0], 1.0, max_relative = 1e-14);
    }

    #[test]
    fn test_wigner_seitz_rs_scaling() {
        // rs scales as rho^(-1/3)
        // If rho doubles, rs should decrease by factor of 2^(-1/3)
        let rho1 = 1.0;
        let rho2 = 2.0;
        let results = run_rs(&[rho1, rho2]);
        let ratio = results[0] / results[1];
        let expected_ratio = 2.0_f64.powf(1.0 / 3.0);
        approx::assert_relative_eq!(ratio, expected_ratio, max_relative = 1e-14);
    }

    #[test]
    fn test_tf_kinetic_rho_one() {
        // tf(1.0) = 0.3 * (3*pi^2)^(2/3) * 1.0
        let kf = (3.0 * std::f64::consts::PI * std::f64::consts::PI).cbrt();
        let expected = 0.3 * kf * kf;
        let results = run_tf(&[1.0]);
        approx::assert_relative_eq!(results[0], expected, max_relative = 1e-14);
    }

    #[test]
    fn test_tf_kinetic_scaling() {
        // tf_kinetic scales as rho^(5/3)
        let rho1 = 1.0;
        let rho2 = 2.0;
        let results = run_tf(&[rho1, rho2]);
        let ratio = results[1] / results[0];
        let expected_ratio = 2.0_f64.powf(5.0 / 3.0);
        approx::assert_relative_eq!(ratio, expected_ratio, max_relative = 1e-14);
    }

    #[test]
    fn test_dimensionless_alpha_uniform() {
        // For uniform electron gas: tau = tau_TF, sigma = 0
        // alpha = (tau_TF - 0) / tau_TF = 1.0
        let rho = 1.0;
        let sigma = 0.0;
        // tau_TF = 0.3 * (3*pi^2)^(2/3) * rho^(5/3)
        let kf = (3.0 * std::f64::consts::PI * std::f64::consts::PI).cbrt();
        let tau_tf = 0.3 * kf * kf * 1.0_f64.powf(5.0 / 3.0);
        let results = run_alpha(&[rho], &[sigma], &[tau_tf]);
        approx::assert_relative_eq!(results[0], 1.0, max_relative = 1e-14);
    }

    #[test]
    fn test_reduced_gradient_uniform() {
        // For uniform electron gas: sigma = 0, so s = 0
        let results = run_reduced_gradient(&[1.0], &[0.0]);
        assert_eq!(results[0], 0.0);
    }

    #[test]
    fn test_reduced_gradient_known() {
        // s = sqrt(sigma) / (2 * KF_CONST * rho^(4/3))
        let rho = 1.0;
        let sigma = 1.0;
        let kf = (3.0 * std::f64::consts::PI * std::f64::consts::PI).cbrt();
        let expected = 1.0 / (2.0 * kf * 1.0_f64.powf(4.0 / 3.0));
        let results = run_reduced_gradient(&[rho], &[sigma]);
        approx::assert_relative_eq!(results[0], expected, max_relative = 1e-14);
    }
}
