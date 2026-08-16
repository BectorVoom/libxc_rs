//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1212/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1212(t11510: f64, t40549: f64, t23754: f64, t3263: f64, t3275: f64, t10810: f64, t3429: f64, t3692: f64, t10935: f64, t2816: f64, t3446: f64, t10928: f64, t122: f64, t3434: f64, t874: f64, t955: f64) -> (f64, f64, f64, f64, f64) {
    let t40551 = 3.0_f64 * t40549 * t11510;
    let t40554 = t3275 * t3263 * t23754 / 4.0_f64;
    let t40556 = t3429 * t10810 * t3692;
    let t40559 = t3446 * t10935 * t2816;
    let t40560 = 0.19211284388664477842e-2_f64 * t40559;
    let t40564 = t3434 * t10928 * t955 * t874 * t122;
    (t40551, t40554, t40556, t40560, t40564)
}
