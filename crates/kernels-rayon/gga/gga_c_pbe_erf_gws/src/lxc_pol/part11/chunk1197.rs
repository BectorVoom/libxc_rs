//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1197/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1197(t3675: f64, t3683: f64, t12929: f64, t19229: f64, t19249: f64, t19439: f64, t25636: f64, t2911: f64, t2912: f64, t34158: f64, t34162: f64, t42806: f64, t48725: f64, t48727: f64, t48728: f64, t48729: f64, t48730: f64, t48731: f64, t48736: f64, t967: f64) -> (f64, f64, f64) {
    let t48823 = t3675 * t3675;
    let t48829 = t3683 * t3683;
    let t48843 = t48725 + t19229 - t19249 + t48727 - t48728 - t48729 + t48730 - t48731 + 0.7152465185185185185e1_f64 * t25636 + 0.2069106e2_f64 * t2911 * t2912 * t967 * t12929 + 0.1379404e2_f64 * t34158 - 0.45980133333333333333e1_f64 * t34162 + t48736 + t19439 - 0.2069106e2_f64 * t42806;
    (t48823, t48829, t48843)
}
