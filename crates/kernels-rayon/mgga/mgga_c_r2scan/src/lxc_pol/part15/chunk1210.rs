//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1210/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1210(t2259: f64, t3574: f64, t3262: f64, t3276: f64, t3579: f64, t36995: f64, t10663: f64, t11523: f64, t37532: f64, t37542: f64, t37556: f64, t37561: f64, t37564: f64, t37569: f64, t40509: f64, t40511: f64, t40513: f64, t40515: f64, t40519: f64, t40521: f64) -> (f64, f64, f64, f64) {
    let t40523 = t3574 * t2259;
    let t40526 = 15.0_f64 / 16.0_f64 * t3262 * t3276 * t40523;
    let t40528 = 5.0_f64 / 16.0_f64 * t3579 * t36995;
    let t40532 = t11523 * t10663 / 2.0_f64;
    let t40533 = t37532 + t40509 - t37542 - 0.36021158228745895953e-3_f64 * t40511 - 0.15243824895787514157e-3_f64 * t40513 + 0.15243824895787514157e-3_f64 * t40515 - t40519 - 0.72042316457491791906e-3_f64 * t40521 + t40526 + t40528 + 0.16260079888840015101e-2_f64 * t37556 + t37561 - 0.30487649791575028314e-3_f64 * t37564 - t37569 + t40532;
    (t40526, t40528, t40532, t40533)
}
