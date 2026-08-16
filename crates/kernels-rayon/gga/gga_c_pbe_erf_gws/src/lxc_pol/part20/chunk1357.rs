//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1357/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1357(t15327: f64, t4414: f64, t12213: f64, t14622: f64, t15207: f64, t2409: f64, t3066: f64, t3207: f64, t3721: f64, t4016: f64, t4052: f64, t43526: f64, t51819: f64, t51825: f64, t57311: f64, t57319: f64, t57324: f64, t57326: f64, t57330: f64, t57332: f64, t57334: f64, t57338: f64, t6781: f64, t9296: f64) -> f64 {
    let t57345 = t4414 * t15327;
    let t57347 = t3066 * t2409 * t12213 * t14622 / 24.0_f64 - t3207 * t2409 * t6781 * t15207 / 16.0_f64 + 5.0_f64 / 768.0_f64 * t57311 + t3066 * t2409 * t43526 * t4016 / 48.0_f64 + t57319 / 3072.0_f64 + t57324 / 768.0_f64 - 7.0_f64 / 144.0_f64 * t57326 + t57330 / 768.0_f64 + t57332 / 24.0_f64 + t57334 / 8.0_f64 - 119.0_f64 / 13824.0_f64 * t51819 + 35.0_f64 / 216.0_f64 * t51825 - 7.0_f64 / 576.0_f64 * t57338 - t3066 * t2409 * t9296 * t4052 * t3721 / 16.0_f64 - 7.0_f64 / 144.0_f64 * t57345;
    t57347
}
