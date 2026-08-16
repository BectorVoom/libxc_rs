//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1173/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1173(t2259: f64, t3574: f64, t113: f64, t36985: f64, t97: f64, t10810: f64, t3429: f64, t3692: f64, t10935: f64, t2816: f64, t3446: f64, t10928: f64, t122: f64, t3434: f64, t874: f64, t955: f64) -> (f64, f64, f64, f64, f64) {
    let t40523 = t3574 * t2259;
    let t40549 = t97 * t36985 * t113;
    let t40556 = t3429 * t10810 * t3692;
    let t40559 = t3446 * t10935 * t2816;
    let t40564 = t3434 * t10928 * t955 * t874 * t122;
    (t40523, t40549, t40556, t40559, t40564)
}
