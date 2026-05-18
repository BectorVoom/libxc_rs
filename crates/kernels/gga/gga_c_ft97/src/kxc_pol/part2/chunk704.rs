//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 704/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk704<F: Float>(t1608: F, t1614: F, t3070: F, t3066: F, t7839: F, t3037: F, t3029: F, t7847: F, t7858: F, t7906: F, t8051: F, t938: F) -> (F, F, F, F, F, F) {
    let t11094 = t1608 * t3070 * t1614;
    let t11095 = t3066 * t7839;
    let t11098 = t3037 * t7839;
    let t11104 = t3029 * t7847;
    let t11109 = t7906 * t7858;
    let t11115 = t8051 * t938;
    (t11094, t11095, t11098, t11104, t11109, t11115)
}
