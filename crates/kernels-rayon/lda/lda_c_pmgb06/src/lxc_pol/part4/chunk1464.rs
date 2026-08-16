//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1464/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1464(t18751: f64, t69: f64, t18754: f64, t18616: f64, t18634: f64, t18637: f64, t18638: f64, t18640: f64, t18644: f64, t18646: f64, t18650: f64, t18737: f64, t18741: f64, t2247: f64, t8263: f64, t8287: f64, t8295: f64, t8441: f64) -> f64 {
    let t18829 = t69 * t18751;
    let t18831 = t69 * t18754;
    let t18835 = -0.7663355555555555_f64 * t8441 - t18616 - 82.76424_f64 * t2247 * t18650 + t8263 + t18634 + t8287 - t8295 - t18637 - t18638 + t18640 + t18644 - t18646 - 1.724255_f64 * t69 * t18737 + 1.1495033333333333_f64 * t18829 + 2.2990066666666666_f64 * t18831 + 10.34553_f64 * t69 * t18741;
    t18835
}
