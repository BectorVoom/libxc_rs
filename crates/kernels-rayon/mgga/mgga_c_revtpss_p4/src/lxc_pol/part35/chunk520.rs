//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 520/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk520(t198: f64, t532: f64, t1907: f64, t4147: f64, t1317: f64, t1857: f64, t1320: f64, t1468: f64, t3833: f64, t1711: f64, t3841: f64, t1856: f64, t749: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5541 = t198 * t532;
    let t5542 = t1907 * t4147;
    let t5545 = t1317 * t1857;
    let t5547 = t1320 * t1857;
    let t5549 = t3833 * t1468;
    let t5557 = t3841 * t1711;
    let t5569 = t1856 * t749;
    (t5541, t5542, t5545, t5547, t5549, t5557, t5569)
}
