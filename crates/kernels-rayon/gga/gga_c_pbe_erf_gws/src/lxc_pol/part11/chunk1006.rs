//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1006/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1006(t12461: f64, t395: f64, t12676: f64, t401: f64, t12481: f64, t12683: f64, t12465: f64, t12473: f64, t10472: f64, t2615: f64, t12555: f64, t17172: f64, t587: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t40251 = t395 * t12461;
    let t40253 = t401 * t12676;
    let t40255 = t395 * t12481;
    let t40260 = t401 * t12683;
    let t40262 = t395 * t12465;
    let t40264 = t395 * t12473;
    let t40321 = t2615 * t10472;
    let t40324 = t587 * t17172 * t12555;
    (t40251, t40253, t40255, t40260, t40262, t40264, t40321, t40324)
}
