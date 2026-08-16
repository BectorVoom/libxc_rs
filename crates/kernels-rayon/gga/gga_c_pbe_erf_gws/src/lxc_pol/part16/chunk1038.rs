//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1038/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1038(t3223: f64, t9376: f64, t2113: f64, t274: f64, t3221: f64, t3220: f64, t1112: f64, t2079: f64, t904: f64, t820: f64, t875: f64, t2306: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9377 = t9376 * t3223;
    let t9380 = t2113 * t274;
    let t9381 = t3221 * t9380;
    let t9382 = t3220 * t9381;
    let t9385 = t2079 * t1112;
    let t9386 = t904 * t9385;
    let t9387 = t875 * t820;
    let t9388 = t2306 * t9387;
    (t9377, t9380, t9381, t9382, t9385, t9386, t9388)
}
