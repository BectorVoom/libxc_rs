//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 821/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk821(t13126: f64, t6801: f64, t1115: f64, t12130: f64, t12138: f64, t12182: f64, t13096: f64, t13105: f64, t13112: f64, t13121: f64, t2503: f64, t3047: f64, t3052: f64, t3917: f64, t3921: f64, t833: f64, t8629: f64, t8793: f64, t9852: f64, t9854: f64, t9879: f64, t9885: f64, t9890: f64, t9907: f64) -> (f64, f64) {
    let t13127 = t13126 * t6801;
    let t13137 = -t1115 * t12138 / 8.0_f64 - t1115 * t9890 / 16.0_f64 + t12130 * t13096 / 32.0_f64 - t3917 * t3047 / 32.0_f64 - t3921 * t3047 / 32.0_f64 + t9907 * t13105 / 32.0_f64 - t1115 * t9885 / 16.0_f64 + t8629 * t13112 / 16.0_f64 - t3917 * t3052 / 16.0_f64 + 7.0_f64 / 48.0_f64 * t9852 + 7.0_f64 / 24.0_f64 * t9854 + t8629 * t13121 / 32.0_f64 + t13127 * t833 / 96.0_f64 + t3917 * t2503 / 32.0_f64 - 7.0_f64 / 48.0_f64 * t9879 + t8793 * t12182 / 8.0_f64 - t3921 * t3052 / 16.0_f64;
    (t13127, t13137)
}
