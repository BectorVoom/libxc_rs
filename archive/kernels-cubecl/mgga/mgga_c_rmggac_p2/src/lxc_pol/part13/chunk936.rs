//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 936/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk936<F: Float>(t2289: F, t36542: F, t34884: F, t8668: F, t8831: F, t8836: F, t8843: F, t2320: F, t35151: F, t34847: F, t1525: F, t236: F, t498: F, t7230: F, t7231: F) -> (F, F, F, F, F, F, F, F) {
    let t40556 = t36542 * t2289;
    let t40558 = t34884 * t8668;
    let t40560 = t34884 * t8831;
    let t40562 = t34884 * t8836;
    let t40564 = t34884 * t8843;
    let t40566 = t35151 * t2320;
    let t40568 = t34847 * t8668;
    let t40573 = t7230 * t7231 * t236 * t1525 * t498;
    (t40556, t40558, t40560, t40562, t40564, t40566, t40568, t40573)
}
