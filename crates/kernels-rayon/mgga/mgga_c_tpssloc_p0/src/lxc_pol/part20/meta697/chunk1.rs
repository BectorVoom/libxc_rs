//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2662/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2662(t25: f64, t54323: f64, t5157: f64, t9874: f64, t5137: f64, t591: f64, t11988: f64, t12061: f64, t1408: f64, t15937: f64, t15940: f64, t16: f64, t2: f64, t3664: f64, t39419: f64, t5134: f64, t514: f64, t53805: f64, t53808: f64, t53814: f64, t53817: f64, t584: f64, t606: f64, t9257: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t26 = t25 <= zeta_threshold;
    let t54324 = 12.0_f64 * t54323;
    let t54325 = t5157 * t9874;
    let t54326 = 0.56968947174242584612e-3_f64 * t54325;
    let t54347 = 32.0_f64 * t5137 * t591;
    let t54349 = piecewise3(t26, 0.0_f64, 40.0_f64 / 81.0_f64 * t39419 * t1408 * t11988 - 16.0_f64 / 9.0_f64 * t12061 * t2 * t53805 - 8.0_f64 / 9.0_f64 * t15937 * t53808 + 8.0_f64 / 3.0_f64 * t3664 * t584 * t606 - 8.0_f64 * t15940 * t53814 + 8.0_f64 / 3.0_f64 * t15940 * t53817 + 4.0_f64 / 9.0_f64 * t5134 * t9257 - 16.0_f64 * t514 * t16 + t54347);
    (t54324, t54326, t54349)
}
