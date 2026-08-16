//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2680/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2680(t1315: f64, t210: f64, t214: f64, t40343: f64, t40347: f64, t40350: f64, t54631: f64, t54633: f64, t54638: f64, t54639: f64, t54644: f64, t56465: f64, t56469: f64, t74355: f64) -> f64 {
    let t74699 = -t40343 + t40347 + t40350 - 0.38888888888888888888e-1_f64 * t54631 + 0.98611111111111111109e-1_f64 * t54633 - t54638 + 0.16851851851851851851e0_f64 * t54639 - 0.14999999999999999999e-1_f64 * t56465 + 0.49999999999999999998e-2_f64 * t56469 - t54644 - 0.16666666666666666666e-2_f64 * t1315 * t210 * t214 * t74355;
    t74699
}
