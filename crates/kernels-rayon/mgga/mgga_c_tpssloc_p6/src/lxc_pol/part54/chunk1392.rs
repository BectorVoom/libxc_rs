//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1392/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1392(t22986: f64, t31338: f64, t86873: f64, t10109: f64, t8562: f64, t33422: f64, t6547: f64, t118877: f64, t118886: f64, t13042: f64, t2054: f64, t25168: f64, t25233: f64, t259: f64, t2597: f64, t26703: f64, t31409: f64, t33395: f64, t33405: f64, t33433: f64, t4147: f64, t4272: f64, t6627: f64, t7087: f64, t798: f64, t8563: f64, t87755: f64, t87810: f64) -> f64 {
    let t121648 = t22986 * t86873 * t31338;
    let t121652 = t10109 * t8562;
    let t121660 = t6547 * t33422;
    let t121668 = 2.0_f64 * t2597 * t33433 + t118877 + 0.16449340668482264365e-1_f64 * t121648 + 2.0_f64 * t7087 * t25233 - 6.0_f64 * t25168 * t121652 * t4272 - t87810 * t2054 + t798 * t33395 * t259 - t13042 * t8563 + 0.19190897446562641759e-1_f64 * t121660 - 6.0_f64 * t87755 * t33405 + t118886 + 2.0_f64 * t4147 * t31409 + 2.0_f64 * t6627 * t26703;
    t121668
}
