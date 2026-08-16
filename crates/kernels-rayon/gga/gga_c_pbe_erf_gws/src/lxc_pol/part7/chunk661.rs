//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 661/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk661(t1860: f64, t401: f64, t1856: f64, t4958: f64, t4963: f64, t1251: f64, t607: f64, t1863: f64, t1857: f64, t177: f64, t572: f64, t191: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5248 = t401 * t1860;
    let t5250 = t1856 * t4958;
    let t5253 = t1856 * t4963;
    let t5256 = t1251 * t607;
    let t5258 = t401 * t1863;
    let t5260 = t401 * t1857;
    let t5263 = 1.0_f64 / t177 / t572;
    let t5264 = t191 * t5263;
    (t5248, t5250, t5253, t5256, t5258, t5260, t5264)
}
