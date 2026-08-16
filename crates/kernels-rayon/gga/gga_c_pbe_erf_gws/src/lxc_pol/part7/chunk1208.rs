//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1208/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1208(t2273: f64, t6717: f64, t2339: f64, t6416: f64, t6667: f64, t2119: f64, t2124: f64, t6106: f64, t20803: f64, t21447: f64, t21452: f64, t21455: f64, t21456: f64, t21462: f64, t2266: f64, t2271: f64, t3247: f64, t6105: f64, t902: f64, t904: f64, t905: f64, t916: f64, t9665: f64) -> (f64, f64) {
    let t21463 = t6717 * t2273;
    let t21465 = t6717 * t2339;
    let t21474 = t6416 * t6667;
    let t21478 = t6106 * t2119 * t2124 / 32.0_f64;
    let t21479 = 3.0_f64 / 512.0_f64 * t2266 * t916 * t904 * t21447 + 7.0_f64 / 576.0_f64 * t21452 - t21455 - 7.0_f64 / 288.0_f64 * t21456 + t21462 + 119.0_f64 / 1152.0_f64 * t21463 + 119.0_f64 / 1152.0_f64 * t21465 + t902 * t905 * t6105 * t2271 / 512.0_f64 - 3.0_f64 / 32.0_f64 * t3247 * t9665 * t20803 - 7.0_f64 / 288.0_f64 * t21474 - t21478;
    (t21478, t21479)
}
