//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 657/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk657<F: Float>(t3352: F, t9117: F, t7230: F, t236: F, t495: F, t618: F, t7231: F, t2061: F, t2868: F, t117: F, t6477: F) -> (F, F, F, F, F, F) {
    let t9118 = t3352 * t9117;
    let t9119 = t7230 * t9118;
    let t9122 = t236 * t618 * t495;
    let t9123 = t7231 * t9122;
    let t9124 = t7230 * t9123;
    let t9126 = t2868 * t2061;
    let t9128 = t6477 * t117;
    (t9118, t9119, t9123, t9124, t9126, t9128)
}
