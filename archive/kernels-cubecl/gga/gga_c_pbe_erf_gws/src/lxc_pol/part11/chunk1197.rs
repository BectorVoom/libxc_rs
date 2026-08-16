//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1197/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1197<F: Float>(t3675: F, t3683: F, t12929: F, t19229: F, t19249: F, t19439: F, t25636: F, t2911: F, t2912: F, t34158: F, t34162: F, t42806: F, t48725: F, t48727: F, t48728: F, t48729: F, t48730: F, t48731: F, t48736: F, t967: F) -> (F, F, F) {
    let t48823 = t3675 * t3675;
    let t48829 = t3683 * t3683;
    let t48843 = t48725 + t19229 - t19249 + t48727 - t48728 - t48729 + t48730 - t48731 + F::cast_from(0.7152465185185185185e1_f64) * t25636 + F::cast_from(0.2069106e2_f64) * t2911 * t2912 * t967 * t12929 + F::cast_from(0.1379404e2_f64) * t34158 - F::cast_from(0.45980133333333333333e1_f64) * t34162 + t48736 + t19439 - F::cast_from(0.2069106e2_f64) * t42806;
    (t48823, t48829, t48843)
}
