//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 817/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk817<F: Float>(t2209: F, t6168: F, t6158: F, t841: F, t218: F, t344: F, t5555: F, t1878: F, t847: F) -> (F, F, F, F, F) {
    let t6169 = t6168 * t2209;
    let t6171 = t841 * t6158;
    let t6174 = t218 * t5555 * t344;
    let t6175 = 0.36514074074074074075e0 * t6174;
    let t6177 = t218 * t1878 * t847;
    (t6169, t6171, t6174, t6175, t6177)
}
