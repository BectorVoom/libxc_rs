//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1142/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1142<F: Float>(t47130: F, t7290: F, t4820: F, t7513: F, t13892: F, t5676: F, t12161: F, t2033: F, t2365: F, t2610: F, t13848: F, t7416: F) -> (F, F, F, F, F) {
    let t47484 = t7290 * t47130;
    let t47486 = t7513 * t4820 * t47484;
    let t47488 = t5676 * t13892;
    let t47492 = t2033 * t2365 * t2610 * t12161;
    let t47494 = t7416 * t13848;
    (t47484, t47486, t47488, t47492, t47494)
}
