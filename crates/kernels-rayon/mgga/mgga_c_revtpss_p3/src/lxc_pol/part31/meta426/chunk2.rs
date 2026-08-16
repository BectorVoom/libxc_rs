//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1530/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1530(t11387: f64, t6109: f64, t934: f64, t11385: f64, t6158: f64, t953: f64, t1622: f64, t4669: f64, t6177: f64, t6174: f64, t2970: f64, t6173: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t19255 = t6109 * t11387;
    let t19256 = t19255 * t934;
    let t19258 = 0.51726012919273400301e3_f64 * t11385 * t19256;
    let t19263 = t6158 * t953;
    let t19266 = t1622 * t4669;
    let t19269 = t6177 * t953;
    let t19272 = t6174 * t953;
    let t19275 = t6173 * t2970;
    (t19258, t19263, t19266, t19269, t19272, t19275)
}
