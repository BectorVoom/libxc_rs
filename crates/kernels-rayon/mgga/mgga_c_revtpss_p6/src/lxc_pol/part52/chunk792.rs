//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 792/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk792(t248: f64, t8486: f64, t3140: f64, t3268: f64, t1078: f64, t1035: f64, t1312: f64, t8460: f64, t196: f64, t2011: f64, t197: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8487 = t8486 * t248;
    let t8515 = t3140 * t3268;
    let t8520 = t3140 * t1078;
    let t8521 = t8520 * t1035;
    let t8563 = t1312 * t8460;
    let t8564 = 2.0_f64 * t8563;
    let t8567 = t2011 * t196;
    let t8568 = t8567 * t197;
    (t8487, t8515, t8521, t8564, t8567, t8568)
}
