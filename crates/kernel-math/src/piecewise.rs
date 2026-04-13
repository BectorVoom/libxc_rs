//! Branch-free piecewise selection functions for CubeCL kernels.
//!
//! These map directly to libxc's `my_piecewise3` and `my_piecewise5` macros,
//! using CubeCL's `select()` for branchless execution on GPU.

use cubecl::prelude::*;

/// Branch-free 2-way select: returns `val_true` if `cond` is true, else `val_false`.
///
/// Maps to libxc's `my_piecewise3(c, x1, x2)`.
/// Note: Both branches are always evaluated (branchless select).
#[cube]
pub fn piecewise3(cond: bool, val_true: f64, val_false: f64) -> f64 {
    select(cond, val_true, val_false)
}

/// Branch-free 3-way select: returns `v1` if `c1`, else `v2` if `c2`, else `v_else`.
///
/// Maps to libxc's `my_piecewise5(c1, x1, c2, x2, x3)`.
/// Note: All branches are always evaluated (nested branchless select).
#[cube]
pub fn piecewise5(c1: bool, v1: f64, c2: bool, v2: f64, v_else: f64) -> f64 {
    select(c1, v1, select(c2, v2, v_else))
}

#[cfg(test)]
mod tests {
    use super::*;
    use cubecl::cpu::{CpuDevice, CpuRuntime};
    use cubecl::Runtime;
    use cubecl::client::ComputeClient;

    #[cube(launch_unchecked)]
    fn test_piecewise3_kernel(
        cond_flag: &Array<f64>,
        val_true: &Array<f64>,
        val_false: &Array<f64>,
        output: &mut Array<f64>,
    ) {
        let idx = ABSOLUTE_POS;
        let cond = cond_flag[idx] > 0.0;
        output[idx] = piecewise3(cond, val_true[idx], val_false[idx]);
    }

    #[cube(launch_unchecked)]
    fn test_piecewise5_kernel(
        c1_flag: &Array<f64>,
        v1: &Array<f64>,
        c2_flag: &Array<f64>,
        v2: &Array<f64>,
        v_else: &Array<f64>,
        output: &mut Array<f64>,
    ) {
        let idx = ABSOLUTE_POS;
        let c1 = c1_flag[idx] > 0.0;
        let c2 = c2_flag[idx] > 0.0;
        output[idx] = piecewise5(c1, v1[idx], c2, v2[idx], v_else[idx]);
    }

    fn make_client() -> ComputeClient<CpuRuntime> {
        let device = CpuDevice;
        CpuRuntime::client(&device)
    }

    fn run_piecewise3(conds: &[f64], val_true: &[f64], val_false: &[f64]) -> Vec<f64> {
        let client = make_client();
        let n = conds.len();

        let h_cond = client.create_from_slice(bytemuck::cast_slice(conds));
        let h_true = client.create_from_slice(bytemuck::cast_slice(val_true));
        let h_false = client.create_from_slice(bytemuck::cast_slice(val_false));
        let h_out = client.empty(n * core::mem::size_of::<f64>());

        unsafe {
            test_piecewise3_kernel::launch_unchecked::<CpuRuntime>(
                &client,
                CubeCount::new_1d(n as u32),
                CubeDim::new_1d(1),
                ArrayArg::from_raw_parts::<f64>(&h_cond, n, 1),
                ArrayArg::from_raw_parts::<f64>(&h_true, n, 1),
                ArrayArg::from_raw_parts::<f64>(&h_false, n, 1),
                ArrayArg::from_raw_parts::<f64>(&h_out, n, 1),
            ).unwrap();
        }

        let bytes = client.read_one(h_out);
        bytemuck::cast_slice(&bytes).to_vec()
    }

    fn run_piecewise5(
        c1: &[f64], v1: &[f64], c2: &[f64], v2: &[f64], v_else: &[f64],
    ) -> Vec<f64> {
        let client = make_client();
        let n = c1.len();

        let h_c1 = client.create_from_slice(bytemuck::cast_slice(c1));
        let h_v1 = client.create_from_slice(bytemuck::cast_slice(v1));
        let h_c2 = client.create_from_slice(bytemuck::cast_slice(c2));
        let h_v2 = client.create_from_slice(bytemuck::cast_slice(v2));
        let h_ve = client.create_from_slice(bytemuck::cast_slice(v_else));
        let h_out = client.empty(n * core::mem::size_of::<f64>());

        unsafe {
            test_piecewise5_kernel::launch_unchecked::<CpuRuntime>(
                &client,
                CubeCount::new_1d(n as u32),
                CubeDim::new_1d(1),
                ArrayArg::from_raw_parts::<f64>(&h_c1, n, 1),
                ArrayArg::from_raw_parts::<f64>(&h_v1, n, 1),
                ArrayArg::from_raw_parts::<f64>(&h_c2, n, 1),
                ArrayArg::from_raw_parts::<f64>(&h_v2, n, 1),
                ArrayArg::from_raw_parts::<f64>(&h_ve, n, 1),
                ArrayArg::from_raw_parts::<f64>(&h_out, n, 1),
            ).unwrap();
        }

        let bytes = client.read_one(h_out);
        bytemuck::cast_slice(&bytes).to_vec()
    }

    #[test]
    fn test_piecewise3_true_branch() {
        let results = run_piecewise3(&[1.0], &[10.0], &[20.0]);
        assert_eq!(results[0], 10.0);
    }

    #[test]
    fn test_piecewise3_false_branch() {
        let results = run_piecewise3(&[-1.0], &[10.0], &[20.0]);
        assert_eq!(results[0], 20.0);
    }

    #[test]
    fn test_piecewise3_multiple() {
        let results = run_piecewise3(
            &[1.0, -1.0, 1.0, -1.0],
            &[1.0, 2.0, 3.0, 4.0],
            &[5.0, 6.0, 7.0, 8.0],
        );
        assert_eq!(results, vec![1.0, 6.0, 3.0, 8.0]);
    }

    #[test]
    fn test_piecewise3_nan_inf_no_leak() {
        // Both branches evaluated (branchless), but select should pick the correct value
        let results = run_piecewise3(&[1.0], &[42.0], &[f64::NAN]);
        assert_eq!(results[0], 42.0);

        let results = run_piecewise3(&[-1.0], &[f64::NAN], &[42.0]);
        assert_eq!(results[0], 42.0);
    }

    #[test]
    fn test_piecewise5_first_branch() {
        let results = run_piecewise5(&[1.0], &[10.0], &[0.0], &[20.0], &[30.0]);
        assert_eq!(results[0], 10.0);
    }

    #[test]
    fn test_piecewise5_second_branch() {
        let results = run_piecewise5(&[-1.0], &[10.0], &[1.0], &[20.0], &[30.0]);
        assert_eq!(results[0], 20.0);
    }

    #[test]
    fn test_piecewise5_else_branch() {
        let results = run_piecewise5(&[-1.0], &[10.0], &[-1.0], &[20.0], &[30.0]);
        assert_eq!(results[0], 30.0);
    }
}
