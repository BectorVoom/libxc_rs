//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1246/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1246<F: Float>(t268: F, t547: F, t6559: F, t22705: F, t22733: F, t22633: F, t22694: F, t3807: F, t6976: F, t12272: F, t12248: F, t2006: F) -> (F, F, F, F, F) {
    let t81228 = t6559 * t547 * t268;
    let t81230 = t81228 * t22705 * t22733;
    let t81234 = t22633 * t6976 * t22694 * t3807;
    let t81238 = t22633 * t6976 * t12272 * t3807;
    let t81243 = t12248 * t2006;
    (t81228, t81230, t81234, t81238, t81243)
}
