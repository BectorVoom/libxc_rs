//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 641/952 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk641<F: Float>(t7231: F, t9110: F, t3351: F, t2283: F, t7720: F, t236: F, t495: F, t551: F, t3352: F, t7230: F, t618: F, t2061: F, t2868: F, t117: F, t6477: F, t2295: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t9111 = t7231 * t9110;
    let t9112 = t3351 * t9111;
    let t9114 = t7720 * t2283;
    let t9117 = t236 * t551 * t495;
    let t9118 = t3352 * t9117;
    let t9119 = t7230 * t9118;
    let t9122 = t236 * t618 * t495;
    let t9123 = t7231 * t9122;
    let t9124 = t7230 * t9123;
    let t9126 = t2868 * t2061;
    let t9128 = t6477 * t117;
    let t9129 = t9128 * t2295;
    (t9111, t9112, t9114, t9118, t9119, t9123, t9124, t9126, t9128, t9129)
}
