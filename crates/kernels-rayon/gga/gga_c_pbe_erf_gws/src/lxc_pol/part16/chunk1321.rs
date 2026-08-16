//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1321/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1321(t2242: f64, t4213: f64, t53508: f64, t53515: f64, t2376: f64, t26617: f64, t4207: f64, t810: f64, t51153: f64, t52309: f64, t53487: f64, t53493: f64, t53498: f64, t53510: f64, t53513: f64, t53517: f64, t53520: f64, t53526: f64, t53529: f64, t6793: f64) -> f64 {
    let t55192 = t2242 * t4213;
    let t55195 = 7.0_f64 / 36.0_f64 * t53508;
    let t55198 = 7.0_f64 / 36.0_f64 * t53515;
    let t55204 = t26617 * t2376 * t4207 * t810;
    let t55208 = -7.0_f64 / 576.0_f64 * t51153 - t53487 / 8.0_f64 - t53493 / 384.0_f64 + t53498 / 384.0_f64 - 35.0_f64 / 432.0_f64 * t55192 - 7.0_f64 / 144.0_f64 * t52309 + t55195 - t53510 / 24.0_f64 + t53513 / 768.0_f64 + t55198 + t53517 / 12.0_f64 - t53520 / 96.0_f64 + 5.0_f64 / 192.0_f64 * t53526 - t6793 * t55204 / 8.0_f64 + t53529 / 384.0_f64;
    t55208
}
