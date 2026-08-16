//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 994/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk994(t1887: f64, t1928: f64, t4810: f64, t802: f64, t1554: f64, t161: f64, t2624: f64, t132: f64, t1547: f64, t2630: f64, t4844: f64, t831: f64) -> (f64, f64, f64, f64, f64) {
    let t17919 = t1887 * t1928;
    let t17921 = t802 * t4810;
    let t17926 = t161 * t1554 * t2624;
    let t17931 = t132 * t1547 * t2630;
    let t17935 = t831 * t4844;
    (t17919, t17921, t17926, t17931, t17935)
}
