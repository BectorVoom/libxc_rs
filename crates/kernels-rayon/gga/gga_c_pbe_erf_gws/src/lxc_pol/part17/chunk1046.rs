//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1046/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1046(t2255: f64, t3258: f64, t6598: f64, t254: f64, t6: f64, t6469: f64, t2081: f64, t2105: f64, t9441: f64, t2182: f64, t274: f64, t1123: f64) -> (f64, f64, f64, f64) {
    let t9478 = t2255 * t3258 * t6598;
    let t9482 = t254 * t6 * t6469;
    let t9483 = t2105 * t2081;
    let t9484 = t9441 * t9483;
    let t9485 = t9482 * t9484;
    let t9488 = t274 * t2182;
    let t9490 = t2255 * t1123 * t9488;
    (t9478, t9484, t9485, t9490)
}
