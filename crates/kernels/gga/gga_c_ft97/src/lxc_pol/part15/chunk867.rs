//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 867/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk867<F: Float>(t39673: F, t1642: F, t1984: F, t525: F, t7954: F, t378: F, t7368: F, t143: F, t37355: F, t137: F, t8906: F, t135: F) -> (F, F, F, F, F, F) {
    let t39674 = F::cast_from(280.0_f64) / F::cast_from(81.0_f64) * t39673;
    let t39693 = t1642 * t1984;
    let t39725 = t7954 * t525;
    let t39749 = t378 * t7368;
    let t39778 = t143 * t37355;
    let t39801 = F::cast_from(1.0_f64) / t8906 / t137;
    let t39802 = t135 * t39801;
    (t39674, t39693, t39725, t39749, t39778, t39802)
}
