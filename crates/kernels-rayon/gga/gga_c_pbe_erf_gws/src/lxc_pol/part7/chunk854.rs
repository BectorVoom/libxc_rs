//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 854/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk854(t16502: f64, t1406: f64, t181: f64, t184: f64, t199: f64, t1885: f64, t5273: f64, t5393: f64, t587: f64, t16481: f64, t16485: f64, t16487: f64, t16490: f64, t16492: f64, t16494: f64, t16498: f64, t16501: f64) -> (f64, f64, f64, f64) {
    let t16503 = 8.0_f64 / 45.0_f64 * t16502;
    let t16504 = t1406 * t1406;
    let t16508 = 4.0_f64 / 5.0_f64 * t16504 * t181 * t184 * t199;
    let t16512 = 16.0_f64 / 15.0_f64 * t587 * t1885 * t5393 * t5273;
    let t16513 = -0.38474813732852776452e0_f64 * t16481 + t16485 - t16487 - t16490 + 0.67090456446662028936e-1_f64 * t16492 - 0.44726970964441352624e-1_f64 * t16494 + t16498 - t16501 - t16503 + t16508 + t16512;
    (t16503, t16508, t16512, t16513)
}
