//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 741/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk741<F: Float>(t2127: F, t7122: F, t2126: F, t6927: F, t115: F, t138: F, t5: F, t6932: F, t6937: F, t2124: F, t2168: F, t3467: F, t3501: F, t6782: F, t6787: F, t6792: F, t6928: F, t7111: F, t7113: F, t7116: F, t7119: F) -> (F, F, F, F, F) {
    let t7123 = t7122 * t2127;
    let t7125 = t2126 * t6927;
    let t7128 = t138 * t115;
    let t7129 = t7128 * t5;
    let t7130 = t7129 * t6932;
    let t7133 = t2126 * t6937;
    let t7136 = F::cast_from(0.18137053605011111023e0_f64) * t2168 * t6928 + F::cast_from(0.18137053605011111023e0_f64) * t2168 * t6782 - F::cast_from(0.5441116081503333307e0_f64) * t3501 * t6787 + F::cast_from(0.13602790203758333267e0_f64) * t3501 * t6792 - F::cast_from(0.16927916698010370288e1_f64) * t7111 + F::cast_from(0.52158968938732547127e0_f64) * t2124 * t7113 - F::cast_from(0.26079484469366273564e0_f64) * t2124 * t7116 + F::cast_from(0.52158968938732547127e0_f64) * t3467 * t7119 - F::cast_from(0.24340852171408521993e1_f64) * t7123 + F::cast_from(0.52158968938732547127e0_f64) * t2124 * t7125 - F::cast_from(0.15647690681619764138e1_f64) * t2124 * t7130 + F::cast_from(0.52158968938732547127e0_f64) * t2124 * t7133;
    (t7125, t7129, t7130, t7133, t7136)
}
