//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2053/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2053<F: Float>(t2635: F, t81886: F, t23041: F, t2681: F, t22690: F, t23122: F, t2553: F, t841: F, t22813: F, t6589: F, t80782: F, t23124: F) -> (F, F, F, F, F) {
    let t81887 = t81886 * t2635;
    let t81889 = t23041 * t2681;
    let t81899 = t23122 * t22690 * t841 * t2553;
    let t81902 = t22813 * t6589 * t80782;
    let t81903 = t81902 * t23124;
    (t81887, t81889, t81899, t81902, t81903)
}
