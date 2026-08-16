//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1327/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1327<F: Float>(t23102: F, t80782: F, t23113: F, t10016: F, t1898: F, t249: F, t23093: F, t281: F, t23046: F, t812: F, t835: F, t2635: F) -> (F, F, F, F) {
    let t81876 = t23102 * t80782;
    let t81877 = t81876 * t23113;
    let t81880 = t10016 * t1898 * t249;
    let t81882 = t23093 * t281;
    let t81883 = t81882 * t23113;
    let t81886 = t812 * t23046 * t835;
    let t81887 = t81886 * t2635;
    (t81877, t81880, t81883, t81887)
}
