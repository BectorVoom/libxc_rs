//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3082/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3082<F: Float>(t136: F, t43761: F, t63420: F, t3297: F, t63311: F, t63315: F, t63368: F, t11219: F, t63372: F, t63378: F, t1113: F, t63402: F) -> (F, F, F, F, F, F, F) {
    let t63918 = t136 * t43761 * t63420;
    let t63921 = t136 * t3297 * t63311;
    let t63924 = t136 * t3297 * t63315;
    let t63927 = t136 * t3297 * t63368;
    let t63930 = t136 * t11219 * t63372;
    let t63933 = t136 * t11219 * t63378;
    let t63936 = t136 * t1113 * t63402;
    (t63918, t63921, t63924, t63927, t63930, t63933, t63936)
}
