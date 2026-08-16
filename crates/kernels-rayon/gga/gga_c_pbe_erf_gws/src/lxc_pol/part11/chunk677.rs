//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 677/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk677(t1383: f64, t992: f64, t1072: f64, t1472: f64, t168: f64, t2893: f64, t501: f64, t485: f64, t974: f64, t1508: f64, t971: f64, t1251: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8058 = t992 * t1383;
    let t8066 = t168 * t1472 * t1072;
    let t8122 = t501 * t2893;
    let t8135 = t485 * t974;
    let t8143 = t1508 * t971;
    let t8144 = t8143 * t1251;
    (t8058, t8066, t8122, t8135, t8143, t8144)
}
