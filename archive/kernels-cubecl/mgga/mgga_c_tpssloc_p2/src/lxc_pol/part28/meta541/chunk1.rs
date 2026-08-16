//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1805/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1805<F: Float>(t23102: F, t80782: F, t23113: F, t23093: F, t281: F, t23046: F, t812: F, t835: F, t2635: F, t23041: F, t2681: F, t22690: F, t23122: F, t2553: F, t841: F) -> (F, F, F, F, F, F, F) {
    let t81876 = t23102 * t80782;
    let t81877 = t81876 * t23113;
    let t81882 = t23093 * t281;
    let t81883 = t81882 * t23113;
    let t81886 = t812 * t23046 * t835;
    let t81887 = t81886 * t2635;
    let t81889 = t23041 * t2681;
    let t81899 = t23122 * t22690 * t841 * t2553;
    (t81876, t81877, t81882, t81883, t81887, t81889, t81899)
}
