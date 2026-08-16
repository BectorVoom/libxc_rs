//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 983/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk983<F: Float>(t800: F, t83232: F, t1208: F, t5014: F, t22134: F, t816: F, t4092: F, t22291: F, t458: F, t22306: F, t1775: F, t22310: F) -> (F, F, F, F, F, F, F) {
    let t83233 = t800 * t83232;
    let t83269 = t5014 * t1208;
    let t83313 = t816 * t22134;
    let t83356 = t4092 * t83232;
    let t83371 = t458 * t22291;
    let t83373 = t458 * t22306;
    let t83381 = t1775 * t22310;
    (t83233, t83269, t83313, t83356, t83371, t83373, t83381)
}
