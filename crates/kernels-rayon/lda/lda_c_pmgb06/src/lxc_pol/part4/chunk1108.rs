//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1108/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1108(t13712: f64, t446: f64, t3259: f64, t813: f64, t1969: f64, t3213: f64, t1423: f64, t4620: f64, t1886: f64, t607: f64, t1427: f64, t5220: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t13713 = t13712 * t446;
    let t13715 = t3259 * t813;
    let t13719 = t3213 * t1969;
    let t13721 = t1423 * t4620;
    let t13726 = t1886 * t607;
    let t13727 = t13726 * t446;
    let t13729 = t5220 * t1427;
    (t13713, t13715, t13719, t13721, t13726, t13727, t13729)
}
