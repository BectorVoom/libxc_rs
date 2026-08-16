//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 981/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk981<F: Float>(t21253: F, t22071: F, t22059: F, t816: F, t2724: F, t21130: F, t2344: F, t1095: F, t70402: F, t22107: F, t8959: F, t22111: F, t39922: F) -> (F, F, F, F, F, F, F) {
    let t82848 = t22071 * t21253;
    let t82851 = t816 * t22059;
    let t82855 = t2724 * t22059;
    let t82988 = t2344 * t21130;
    let t83049 = t70402 * t1095;
    let t83084 = t8959 * t22107;
    let t83086 = t39922 * t22111;
    (t82848, t82851, t82855, t82988, t83049, t83084, t83086)
}
