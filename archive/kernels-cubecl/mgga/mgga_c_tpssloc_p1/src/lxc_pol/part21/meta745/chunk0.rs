//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2614/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2614<F: Float>(t11734: F, t15548: F, t1174: F, t14749: F, t3431: F, t1222: F, t15723: F, t11738: F, t13969: F, t15534: F, t3514: F, t53371: F) -> (F, F, F, F, F) {
    let t53378 = t11734 * t15548;
    let t53387 = t1174 * t3431 * t14749;
    let t53389 = t15723 * t1222;
    let t53397 = t11738 * t13969 * t15534;
    let t53399 = t53371 * t3514;
    (t53378, t53387, t53389, t53397, t53399)
}
