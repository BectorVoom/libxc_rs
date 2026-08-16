//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1216/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1216(t1134: f64, t13187: f64, t1113: f64, t13172: f64, t13397: f64, t2253: f64, t2255: f64, t2343: f64, t343: f64, t3747: f64, t3781: f64, t44372: f64, t49239: f64, t49279: f64, t49281: f64, t49283: f64, t49285: f64, t49295: f64, t49299: f64, t902: f64, t905: f64, t9343: f64) -> (f64, f64) {
    let t49305 = t1134 * t13187;
    let t49313 = -t49279 - t49281 - t49283 + t49285 - t2253 * t2255 * t3781 * t13397 * t343 / 128.0_f64 + t49295 + t49299 - 7.0_f64 / 576.0_f64 * t44372 + t902 * t905 * t13172 * t3747 / 512.0_f64 - 5.0_f64 / 32.0_f64 * t2343 * t9343 * t49305 + t902 * t905 * t1113 * t49239 / 1536.0_f64;
    (t49305, t49313)
}
