//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 499/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk499(t1284: f64, t487: f64, t1209: f64, t1269: f64, t473: f64, t3140: f64, t3596: f64, t460: f64, t1243: f64, t498: f64, t1330: f64, t72: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3754 = t1284 * t487;
    let t3755 = t1209 * t3754;
    let t3759 = t473 * t1269;
    let t3766 = t3140 * t3596;
    let t3767 = t460 * t3766;
    let t3781 = t3140 * t1243;
    let t3782 = t460 * t3781;
    let t3800 = t498 * t498;
    let t3801 = 1.0_f64 / t3800;
    let t3825 = t1330 * t72;
    (t3755, t3759, t3766, t3767, t3781, t3782, t3800, t3801, t3825)
}
