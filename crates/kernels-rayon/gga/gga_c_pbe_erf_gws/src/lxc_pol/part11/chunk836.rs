//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 836/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk836(t11447: f64, t11782: f64, t3134: f64, t1105: f64, t337: f64, t3791: f64, t2147: f64, t3116: f64, t11787: f64, t9035: f64, t3763: f64, t3781: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t13238 = 7.0_f64 / 48.0_f64 * t11447;
    let t13240 = t11782 * t3134 / 32.0_f64;
    let t13242 = t337 * t3791 * t1105;
    let t13243 = t2147 * t13242;
    let t13245 = t3116 * t13243 / 16.0_f64;
    let t13247 = t9035 * t11787 / 16.0_f64;
    let t13248 = t3781 * t3763;
    (t13238, t13240, t13242, t13243, t13245, t13247, t13248)
}
