//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 708/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk708<F: Float>(t1771: F, t926: F, t3044: F, t458: F, t3047: F, t14: F, t7741: F, t12: F, t9: F, t3053: F, t11008: F, t7954: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t11167 = t1771 * t926;
    let t11169 = t458 * t3044;
    let t11170 = F::new(4.0) / F::new(27.0) * t11169;
    let t11171 = t458 * t3047;
    let t11172 = F::new(4.0) / F::new(9.0) * t11171;
    let t11174 = F::new(1.0) / t14 / t7741;
    let t11175 = t12 * t11174;
    let t11176 = t9 * t11175;
    let t11177 = t11176 * t3053;
    let t11179 = t7954 * t11008;
    (t11167, t11169, t11170, t11171, t11172, t11174, t11175, t11176, t11177, t11179)
}
