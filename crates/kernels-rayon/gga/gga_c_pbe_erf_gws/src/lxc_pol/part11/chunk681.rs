//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 681/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk681(t1076: f64, t39: f64, t169: f64, t2994: f64, t700: f64, t784: f64, t991: f64, t242: f64, t1086: f64, t1383: f64, t1371: f64, t2948: f64, t553: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8347 = t39 * t1076;
    let t8357 = t169 * t2994 * t700;
    let t8361 = t784 * t991;
    let t8363 = t169 * t8361 * t242;
    let t8373 = t169 * t1086 * t1383;
    let t8387 = t2948 * t1371 * t553;
    (t8347, t8357, t8361, t8363, t8373, t8387)
}
