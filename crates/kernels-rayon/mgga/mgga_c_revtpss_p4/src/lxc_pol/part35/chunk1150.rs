//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1150/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1150(t103363: f64, t25299: f64, t2439: f64, t780: f64, t785: f64, t7997: f64, t25305: f64, t2435: f64, t28390: f64, t102993: f64, t25411: f64, t2470: f64, t28359: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t103364 = t25299 * t103363;
    let t103370 = t2439 * t785 * t7997 * t780;
    let t103394 = t25305 * t103363;
    let t103400 = t2435 * t28390;
    let t103404 = t25411 * t102993;
    let t103421 = t28359 * t2470;
    (t103364, t103370, t103394, t103400, t103404, t103421)
}
