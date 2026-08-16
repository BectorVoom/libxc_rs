//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1210/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1210<F: Float>(t2259: F, t3574: F, t3262: F, t3276: F, t3579: F, t36995: F, t10663: F, t11523: F, t37532: F, t37542: F, t37556: F, t37561: F, t37564: F, t37569: F, t40509: F, t40511: F, t40513: F, t40515: F, t40519: F, t40521: F) -> (F, F, F, F) {
    let t40523 = t3574 * t2259;
    let t40526 = F::cast_from(15.0_f64) / F::cast_from(16.0_f64) * t3262 * t3276 * t40523;
    let t40528 = F::cast_from(5.0_f64) / F::cast_from(16.0_f64) * t3579 * t36995;
    let t40532 = t11523 * t10663 / F::cast_from(2.0_f64);
    let t40533 = t37532 + t40509 - t37542 - F::cast_from(0.36021158228745895953e-3_f64) * t40511 - F::cast_from(0.15243824895787514157e-3_f64) * t40513 + F::cast_from(0.15243824895787514157e-3_f64) * t40515 - t40519 - F::cast_from(0.72042316457491791906e-3_f64) * t40521 + t40526 + t40528 + F::cast_from(0.16260079888840015101e-2_f64) * t37556 + t37561 - F::cast_from(0.30487649791575028314e-3_f64) * t37564 - t37569 + t40532;
    (t40526, t40528, t40532, t40533)
}
