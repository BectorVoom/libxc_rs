//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1063/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1063<F: Float>(t2033: F, t3925: F, t10159: F, t10164: F, t10169: F, t10174: F, t2024: F, t2027: F, t684: F, t687: F, t8224: F, t8450: F, t8452: F, t8454: F, t8462: F, t8476: F, t8479: F, t8491: F, t8501: F) -> (F, F) {
    let t10176 = t2033 * t3925;
    let t10180 = t8224 / F::cast_from(48.0_f64) - t8450 - t8452 + t8454 / F::cast_from(48.0_f64) - t8462 - t8476 + t8479 / F::cast_from(48.0_f64) - t8491 - t8501 - t684 * t687 * t10159 / F::cast_from(64.0_f64) - t684 * t687 * t10164 / F::cast_from(32.0_f64) - t684 * t687 * t10169 / F::cast_from(64.0_f64) - t10174 / F::cast_from(144.0_f64) - t2024 * t2027 * t10176 / F::cast_from(48.0_f64);
    (t10176, t10180)
}
