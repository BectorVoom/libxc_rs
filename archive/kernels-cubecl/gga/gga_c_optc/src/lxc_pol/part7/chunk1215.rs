//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1215/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1215<F: Float>(t309: F, t7898: F, t300: F, t2587: F, t7886: F, t2586: F, t7866: F, t893: F, t7298: F, t896: F, t22015: F, t894: F) -> (F, F, F, F, F, F) {
    let t25071 = t7898 * t309;
    let t25072 = t300 * t25071;
    let t25075 = t7886 * t2587;
    let t25077 = t2586 * t7866;
    let t25078 = t893 * t25077;
    let t25085 = t896 * t7298;
    let t25087 = t894 * t25085 * t22015;
    (t25071, t25072, t25075, t25077, t25078, t25087)
}
