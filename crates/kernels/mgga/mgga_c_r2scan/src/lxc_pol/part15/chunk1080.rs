//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1080/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1080<F: Float>(t10810: F, t3429: F, t3692: F, t10935: F, t2816: F, t3446: F, t10928: F, t122: F, t3434: F, t874: F, t955: F, t32094: F, t792: F, t37327: F, t4176: F, t11502: F, t37346: F) -> (F, F, F, F, F) {
    let t40556 = t3429 * t10810 * t3692;
    let t40559 = t3446 * t10935 * t2816;
    let t40560 = 0.19211284388664477842e-2 * t40559;
    let t40564 = t3434 * t10928 * t955 * t874 * t122;
    let t40566 = t32094 * t792;
    let t40569 = 15.0 / 8.0 * t37327 * t4176 * t40566;
    let t40571 = 3.0 / 4.0 * t37346 * t11502;
    (t40556, t40560, t40564, t40569, t40571)
}
