//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 992/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk992(t12516: f64, t6646: f64, t161: f64, t489: f64, t6832: f64, t188: f64, t539: f64, t6716: f64, t1409: f64, t2414: f64, t1912: f64, t5194: f64) -> (f64, f64, f64, f64, f64) {
    let t17738 = t12516 * t6646;
    let t17771 = t161 * t489 * t6832;
    let t17787 = t6716 * t539 * t188;
    let t17790 = t2414 * t1409 * t188;
    let t17801 = t5194 * t1912;
    (t17738, t17771, t17787, t17790, t17801)
}
