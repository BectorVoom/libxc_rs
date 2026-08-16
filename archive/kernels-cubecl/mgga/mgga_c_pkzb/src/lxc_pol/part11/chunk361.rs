//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 361/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk361<F: Float>(t55: F, t58: F, t63: F, t50: F, t64: F, t66: F, t80: F) -> (F, F, F) {
    let t1475 = F::cast_from(1.0_f64) / t58 / t55 * t63;
    let t1476 = t64 * t50;
    let t1478 = F::cast_from(1.0_f64) / t66 / t80;
    (t1475, t1476, t1478)
}
