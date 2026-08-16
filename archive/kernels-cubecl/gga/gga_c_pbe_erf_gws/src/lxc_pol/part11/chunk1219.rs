//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1219/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1219<F: Float>(t44577: F, t3793: F, t45410: F, t44530: F, t44606: F, t1149: F, t11700: F, t12024: F, t21399: F, t2312: F, t3748: F, t3862: F, t44589: F, t44600: F, t44604: F, t45201: F, t6579: F) -> (F, F, F, F, F) {
    let t49347 = F::cast_from(7.0_f64) / F::cast_from(24.0_f64) * t44577;
    let t49356 = t45410 * t3793 / F::cast_from(32.0_f64);
    let t49362 = t44530 * t3793 / F::cast_from(16.0_f64);
    let t49364 = F::cast_from(7.0_f64) / F::cast_from(36.0_f64) * t44606;
    let t49368 = -t49347 - F::cast_from(7.0_f64) / F::cast_from(96.0_f64) * t44589 + F::cast_from(5.0_f64) / F::cast_from(64.0_f64) * t6579 * t12024 * t3862 - t2312 * t11700 * t3748 / F::cast_from(64.0_f64) - t49356 + F::cast_from(7.0_f64) / F::cast_from(48.0_f64) * t44600 - F::cast_from(5.0_f64) / F::cast_from(16.0_f64) * t21399 * t45201 * t1149 - t49362 + F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t44604 + t49364 + F::cast_from(5.0_f64) / F::cast_from(64.0_f64) * t6579 * t12024 * t3748;
    (t49347, t49356, t49362, t49364, t49368)
}
