//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 909/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk909(t4143: f64, t607: f64, t1710: f64, t1727: f64, t1512: f64, t1548: f64, t2857: f64, t432: f64, t1441: f64, t3213: f64, t1423: f64, t3191: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10083 = t4143 * t607;
    let t10085 = t1727 * t1710;
    let t10087 = t1512 * t1548;
    let t10089 = t432 * t2857;
    let t10099 = t3213 * t1441;
    let t10101 = t1423 * t3191;
    (t10083, t10085, t10087, t10089, t10099, t10101)
}
