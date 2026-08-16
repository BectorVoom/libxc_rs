//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1028/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1028(t11514: f64, t2171: f64, t2345: f64, t6229: f64, t11464: f64, t3140: f64, t3235: f64, t3752: f64, t810: f64, t1123: f64, t2255: f64, t814: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11516 = t2345 * t11514 * t2171;
    let t11519 = 35.0_f64 / 432.0_f64 * t6229;
    let t11521 = t3235 * t11464 * t3140;
    let t11524 = t3752 * t810;
    let t11525 = t1123 * t11524;
    let t11526 = t2255 * t11525;
    let t11529 = t3752 * t814;
    (t11516, t11519, t11521, t11525, t11526, t11529)
}
