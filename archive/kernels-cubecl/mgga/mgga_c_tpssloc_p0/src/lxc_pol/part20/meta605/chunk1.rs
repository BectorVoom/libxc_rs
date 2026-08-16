//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2188/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2188<F: Float>(t11789: F, t1227: F, t248: F, t3252: F, t3248: F, t11665: F, t11698: F, t11683: F, t11697: F, t3577: F, t11673: F, t11678: F, t11679: F) -> (F, F, F, F, F, F) {
    let t44972 = t1227 * t248 * t11789 * t3252;
    let t44976 = t1227 * t248 * t11789 * t3248;
    let t44982 = t11665 * t11698;
    let t44985 = t3577 * t11697 * t11683;
    let t44988 = t3577 * t11697 * t11673;
    let t44991 = t11678 * t11697 * t11679;
    (t44972, t44976, t44982, t44985, t44988, t44991)
}
