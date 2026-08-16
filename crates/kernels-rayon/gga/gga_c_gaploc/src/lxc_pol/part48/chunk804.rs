//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 804/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk804(t2325: f64, t31501: f64, t882: f64, t883: f64, t2321: f64, t34604: f64, t9074: f64, t10687: f64, t2554: f64, t7064: f64, t13200: f64, t29439: f64) -> (f64, f64, f64, f64) {
    let t42889 = t882 * t2325 * t883 * t31501;
    let t42898 = t9074 * t34604 * t2321;
    let t42931 = t7064 * t10687 * t2554;
    let t42933 = t29439 * t13200;
    (t42889, t42898, t42931, t42933)
}
