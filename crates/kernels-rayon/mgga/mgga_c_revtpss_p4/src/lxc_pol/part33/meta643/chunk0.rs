//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2092/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2092(t17288: f64, t2142: f64, t5216: f64, t1209: f64, t2143: f64, t26852: f64, t5378: f64, t29083: f64, t3636: f64, t1234: f64, t29082: f64, t17620: f64, t26870: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t104521 = t17288 * t2142;
    let t104524 = t5216 * t2142;
    let t104549 = t1209 * t2143;
    let t104624 = 0.3811023832717309953e-3_f64 * t26852 * t5378;
    let t104626 = 0.20325460441158986416e-2_f64 * t29083 * t3636;
    let t104636 = t1234 * t29082;
    let t104640 = 0.57165357490759649296e-3_f64 * t26870 * t17620;
    (t104521, t104524, t104549, t104624, t104626, t104636, t104640)
}
