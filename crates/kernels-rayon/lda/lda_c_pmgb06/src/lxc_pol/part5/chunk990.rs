//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 990/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk990(t1377: f64, t2676: f64, t97: f64, t1555: f64, t2563: f64, t1423: f64, t6297: f64, t5108: f64, t851: f64, t5118: f64, t822: f64, t2599: f64, t3458: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t17550 = t2676 * t97 * t1377;
    let t17563 = t2563 * t1555;
    let t17577 = t1423 * t6297;
    let t17598 = t5108 * t851;
    let t17617 = t5118 * t822;
    let t17621 = t3458 * t2599;
    (t17550, t17563, t17577, t17598, t17617, t17621)
}
