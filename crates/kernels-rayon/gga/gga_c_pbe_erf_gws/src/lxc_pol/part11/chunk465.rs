//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 465/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk465(t506: f64, t9: f64, t1076: f64, t169: f64, t301: f64, t784: f64, t285: f64, t545: f64, t991: f64, t281: f64, t1083: f64, t751: f64) -> (f64, f64, f64, f64, f64) {
    let t2912 = t9 * t506;
    let t2926 = t169 * t784 * t1076 * t301;
    let t2936 = t991 * t545 * t285;
    let t2937 = t281 * t2936;
    let t2939 = t751 * t1083;
    (t2912, t2926, t2936, t2937, t2939)
}
