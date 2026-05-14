//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 634/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk634<F: Float>(t3352: F, t9117: F, t7230: F, t236: F, t495: F, t618: F, t7231: F, t2061: F, t2868: F, t117: F, t6477: F, t2295: F, t7204: F, t8902: F, t7192: F, t8906: F) -> (F, F, F, F, F, F, F, F, F) {
    let t9118 = t3352 * t9117;
    let t9119 = t7230 * t9118;
    let t9122 = t236 * t618 * t495;
    let t9123 = t7231 * t9122;
    let t9124 = t7230 * t9123;
    let t9126 = t2868 * t2061;
    let t9128 = t6477 * t117;
    let t9129 = t9128 * t2295;
    let t9133 = t7204 * t8902;
    let t9135 = t7192 * t8906;
    (t9118, t9119, t9123, t9124, t9126, t9128, t9129, t9133, t9135)
}
