//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1208/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1208<F: Float>(t24927: F, t20407: F, t2552: F, t5147: F, t2531: F, t551: F, t574: F, t6343: F, t2184: F, t2634: F, t1592: F, t2612: F, t1584: F, t7597: F, t2097: F, t2649: F, t571: F) -> (F, F, F, F, F, F, F) {
    let t24928 = 0.2037639021386884617e0 * t24927;
    let t24948 = t5147 * t20407 * t2552;
    let t24962 = t574 * t551 * t6343 * t2531;
    let t24963 = 0.12713391885412927226e1 * t24962;
    let t24966 = t2184 * t551 * t6343 * t2634;
    let t24967 = 0.25426783770825854452e1 * t24966;
    let t24970 = t1592 * t551 * t6343 * t2612;
    let t24971 = 0.38140175656238781678e1 * t24970;
    let t24972 = t1584 * t7597;
    let t24973 = 0.12713391885412927226e1 * t24972;
    let t24994 = t571 * t2649 * t2097;
    (t24928, t24948, t24963, t24967, t24971, t24973, t24994)
}
