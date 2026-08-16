//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 920/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk920(t642: f64, t794: f64, t113: f64, t301: f64, t8131: f64, t122: f64, t4182: f64, t886: f64, t199: f64, t5567: f64, t1135: f64, t868: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11676 = t642 * t794;
    let t11678 = t11676 * t113 * t301;
    let t11694 = 48.0_f64 * t8131;
    let t11726 = t122 * t4182 * t886;
    let t11731 = t5567 * t199;
    let t11733 = t1135 * t868;
    (t11676, t11678, t11694, t11726, t11731, t11733)
}
