//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 776/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk776(t12915: f64, t828: f64, t1242: f64, t11239: f64, t1243: f64, t3596: f64, t1275: f64, t4171: f64, t602: f64, t1466: f64, t2246: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12916 = t828 * t12915;
    let t13037 = t1242 * t1242;
    let t13038 = 1.0_f64 / t13037;
    let t13126 = t11239 * t1243;
    let t13141 = t11239 * t3596;
    let t13180 = t1275 * t1275;
    let t13181 = 1.0_f64 / t13180;
    let t13269 = t4171 * t602;
    let t13272 = t1466 * t2246;
    (t12916, t13038, t13126, t13141, t13180, t13181, t13269, t13272)
}
