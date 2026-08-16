//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1125/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1125(t2190: f64, t810: f64, t824: f64, t2147: f64, t337: f64, t6325: f64, t6326: f64, t6705: f64, t2120: f64, t2112: f64, t6345: f64, t6319: f64, t6535: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t20206 = t2190 * t810;
    let t20207 = t824 * t20206;
    let t20215 = t6325 * t2147 * t337 * t6326 * t810 / 4.0_f64;
    let t20219 = t2147 * t337 * t6705 * t810;
    let t20221 = t2120 * t20219 / 12.0_f64;
    let t20222 = t6345 * t2112;
    let t20228 = t6319 * t6535 / 6.0_f64;
    (t20206, t20207, t20215, t20221, t20222, t20228)
}
