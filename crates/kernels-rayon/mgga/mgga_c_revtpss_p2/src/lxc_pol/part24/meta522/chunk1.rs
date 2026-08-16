//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1552/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1552(t1261: f64, t24248: f64, t247: f64, t3634: f64, t21233: f64, t5381: f64, t17401: f64, t20926: f64, t24770: f64, t73: f64, t12916: f64, t24752: f64, t3718: f64) -> (f64, f64, f64, f64, f64) {
    let t82603 = t1261 * t247 * t3634 * t24248;
    let t82656 = t5381 * t21233;
    let t82678 = t17401 * t20926;
    let t82725 = t24770 * t73;
    let t82749 = t3718 * t12916 * t24752;
    (t82603, t82656, t82678, t82725, t82749)
}
