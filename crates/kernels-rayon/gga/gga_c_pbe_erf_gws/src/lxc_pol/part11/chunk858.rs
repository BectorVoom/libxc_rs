//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 858/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk858(t11912: f64, t11922: f64, t12054: f64, t3180: f64, t3772: f64, t5: f64, t337: f64, t2121: f64, t3116: f64, t13347: f64, t2170: f64, t3131: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13485 = 7.0_f64 / 96.0_f64 * t11912;
    let t13486 = 7.0_f64 / 16.0_f64 * t11922;
    let t13488 = t12054 * t3180 / 16.0_f64;
    let t13489 = t5 * t3772;
    let t13490 = t337 * t13489;
    let t13491 = t2121 * t13490;
    let t13493 = t3116 * t13491 / 96.0_f64;
    let t13496 = t2170 * t3131 * t13347;
    (t13485, t13486, t13488, t13489, t13490, t13491, t13493, t13496)
}
