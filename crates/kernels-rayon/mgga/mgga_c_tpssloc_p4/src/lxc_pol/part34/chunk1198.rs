//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1198/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1198(t6888: f64, t7691: f64, t97511: f64, t1985: f64, t20601: f64, t214: f64, t225: f64, t567: f64, t22685: f64, t26193: f64, t28191: f64, t28232: f64) -> (f64, f64, f64, f64) {
    let t107250 = t6888 * t97511 * t7691;
    let t107260 = t1985 * t214 * t20601 * t225 * t567;
    let t107265 = t22685 * t26193 * t28191;
    let t107268 = t1985 * t26193 * t28232;
    (t107250, t107260, t107265, t107268)
}
