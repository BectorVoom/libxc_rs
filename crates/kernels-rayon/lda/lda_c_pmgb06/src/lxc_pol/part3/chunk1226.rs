//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1226/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1226(t1147: f64, t117: f64, t123: f64, t859: f64, t2791: f64, t795: f64, t415: f64, t5543: f64, t1347: f64, t1799: f64, t315: f64, t5689: f64) -> (f64, f64, f64, f64, f64) {
    let t14527 = t123 * t1147 * t859 * t117;
    let t14529 = t795 * t2791;
    let t14533 = t5543 * t415;
    let t14535 = t1799 * t1347;
    let t14536 = 0.09451622166942335_f64 * t14535;
    let t14539 = t123 * t315 * t5689 * t117;
    (t14527, t14529, t14533, t14536, t14539)
}
