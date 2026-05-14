//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 853/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk853<F: Float>(t2215: F, t3747: F, t836: F, t841: F, t9798: F, t218: F, t3757: F, t675: F) -> (F, F, F) {
    let t9811 = t2215 * t3747;
    let t9812 = t9811 * t836;
    let t9814 = t841 * t9798;
    let t9819 = t218 * t675 * t3757;
    (t9812, t9814, t9819)
}
