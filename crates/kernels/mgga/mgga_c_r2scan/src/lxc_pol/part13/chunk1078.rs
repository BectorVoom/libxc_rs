//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1078/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1078<F: Float>(t3579: F, t36995: F, t10663: F, t11523: F, t37532: F, t37542: F, t37556: F, t37561: F, t37564: F, t37569: F, t40509: F, t40511: F, t40513: F, t40515: F, t40519: F, t40521: F, t40526: F) -> (F, F, F) {
    let t40528 = 5.0 / 16.0 * t3579 * t36995;
    let t40532 = t11523 * t10663 / 2.0;
    let t40533 = t37532 + t40509 - t37542 - 0.36021158228745895953e-3 * t40511 - 0.15243824895787514157e-3 * t40513 + 0.15243824895787514157e-3 * t40515 - t40519 - 0.72042316457491791906e-3 * t40521 + t40526 + t40528 + 0.16260079888840015101e-2 * t37556 + t37561 - 0.30487649791575028314e-3 * t37564 - t37569 + t40532;
    (t40528, t40532, t40533)
}
