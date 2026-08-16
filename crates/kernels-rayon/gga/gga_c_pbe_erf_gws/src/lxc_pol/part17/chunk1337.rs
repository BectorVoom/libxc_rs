//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1337/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1337(t3972: f64, t3975: f64, t9574: f64, t1161: f64, t353: f64, t51084: f64, t859: f64, t4183: f64, t4386: f64, t810: f64, t1173: f64, t9203: f64) -> (f64, f64, f64, f64) {
    let t54541 = t3972 * t3975 * t9574;
    let t54545 = t859 * t353 * t51084 * t1161;
    let t54550 = t4386 * t353 * t4183 * t810;
    let t54561 = t1173 * t9203;
    (t54541, t54545, t54550, t54561)
}
