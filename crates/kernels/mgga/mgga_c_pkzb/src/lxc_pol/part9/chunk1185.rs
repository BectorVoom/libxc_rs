//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1185/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1185<F: Float>(t2865: F, t5842: F, t730: F, t2866: F, t5754: F, t2848: F, t5490: F, t7227: F, t1083: F, t5802: F, t17660: F, t683: F) -> (F, F, F, F) {
    let t20652 = F::new(0.11696447245269292414e1) * t730 * t2865 * t5842;
    let t20654 = F::new(0.35089341735807877242e1) * t5754 * t2866;
    let t20658 = F::new(0.30762056574649219973e4) * t730 * t5490 * t2848 * t7227;
    let t20659 = t5802 * t1083;
    let t20662 = F::new(0.1551780387578202009e4) * t20659 * t17660 * t683;
    (t20652, t20654, t20658, t20662)
}
