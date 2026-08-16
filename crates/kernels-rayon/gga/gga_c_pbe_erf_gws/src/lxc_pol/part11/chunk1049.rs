//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1049/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1049(t2083: f64, t3373: f64, t1114: f64, t346: f64, t13468: f64, t8967: f64, t13595: f64, t6416: f64, t13126: f64, t20585: f64, t13387: f64, t6203: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t44900 = t2083 * t3373;
    let t44902 = t1114 * t44900 * t346;
    let t44949 = t8967 * t13468;
    let t44970 = t6416 * t13595;
    let t44972 = t13126 * t20585;
    let t44977 = t6203 * t13387;
    (t44900, t44902, t44949, t44970, t44972, t44977)
}
