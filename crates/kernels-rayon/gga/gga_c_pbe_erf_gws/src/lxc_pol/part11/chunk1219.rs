//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1219/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1219(t44577: f64, t3793: f64, t45410: f64, t44530: f64, t44606: f64, t1149: f64, t11700: f64, t12024: f64, t21399: f64, t2312: f64, t3748: f64, t3862: f64, t44589: f64, t44600: f64, t44604: f64, t45201: f64, t6579: f64) -> (f64, f64, f64, f64, f64) {
    let t49347 = 7.0_f64 / 24.0_f64 * t44577;
    let t49356 = t45410 * t3793 / 32.0_f64;
    let t49362 = t44530 * t3793 / 16.0_f64;
    let t49364 = 7.0_f64 / 36.0_f64 * t44606;
    let t49368 = -t49347 - 7.0_f64 / 96.0_f64 * t44589 + 5.0_f64 / 64.0_f64 * t6579 * t12024 * t3862 - t2312 * t11700 * t3748 / 64.0_f64 - t49356 + 7.0_f64 / 48.0_f64 * t44600 - 5.0_f64 / 16.0_f64 * t21399 * t45201 * t1149 - t49362 + 7.0_f64 / 288.0_f64 * t44604 + t49364 + 5.0_f64 / 64.0_f64 * t6579 * t12024 * t3748;
    (t49347, t49356, t49362, t49364, t49368)
}
