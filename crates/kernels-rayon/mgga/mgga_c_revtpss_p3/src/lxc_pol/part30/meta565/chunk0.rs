//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2012/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2012(t2438: f64, t837: f64, t93172: f64, t93170: f64, t25305: f64, t92894: f64, t786: f64, t92889: f64, t7060: f64, t2434: f64, t25377: f64, t25431: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t93173 = t2438 * t837;
    let t93174 = t93172 * t93173;
    let t93175 = t93170 * t93174;
    let t93177 = t25305 * t92894;
    let t93179 = t786 * t92889;
    let t93180 = t93179 * t7060;
    let t93182 = t2434 * t837;
    let t93183 = t25377 * t93182;
    let t93184 = t25431 * t93183;
    (t93174, t93175, t93177, t93180, t93183, t93184)
}
