//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1493/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1493(t116926: f64, t8312: f64, t116929: f64, t8316: f64, t31027: f64, t31146: f64, t31032: f64, t31153: f64, t31150: f64, t10241: f64, t104: f64, t116912: f64, t31139: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t117184 = t116926 * t8312;
    let t117186 = t116929 * t8316;
    let t117188 = t31027 * t31146;
    let t117190 = t31032 * t31153;
    let t117198 = t31032 * t31150;
    let t117218 = t104 * t10241;
    let t117226 = t116912 * t31139;
    (t117184, t117186, t117188, t117190, t117198, t117218, t117226)
}
