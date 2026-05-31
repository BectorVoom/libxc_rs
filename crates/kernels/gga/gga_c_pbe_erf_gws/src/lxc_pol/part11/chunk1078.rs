//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1078/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1078<F: Float>(t10938: F, t1827: F, t3346: F, t587: F, t32260: F, t3342: F, t5543: F, t39870: F, t41840: F, t997: F, t39883: F, t39886: F) -> (F, F, F, F, F, F) {
    let t47359 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t587 * t1827 * t10938 * t3346;
    let t47363 = F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t587 * t5543 * t32260 * t3342;
    let t47364 = F::cast_from(16.0_f64) / F::cast_from(15.0_f64) * t39870;
    let t47366 = F::cast_from(16.0_f64) / F::cast_from(15.0_f64) * t41840 * t997;
    let t47368 = F::cast_from(32.0_f64) / F::cast_from(15.0_f64) * t39883;
    let t47369 = F::cast_from(32.0_f64) / F::cast_from(15.0_f64) * t39886;
    (t47359, t47363, t47364, t47366, t47368, t47369)
}
