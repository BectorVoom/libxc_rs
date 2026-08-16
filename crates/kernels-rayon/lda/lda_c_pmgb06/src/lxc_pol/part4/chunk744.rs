//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 744/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk744(t1547: f64, t823: f64, t132: f64, t409: f64, t495: f64, t177: f64, t497: f64, t161: f64, t1554: f64, t852: f64, t1083: f64, t1825: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4836 = t1547 * t823;
    let t4837 = t132 * t4836;
    let t4838 = t4837 / 135.0_f64;
    let t4839 = t409 * t495;
    let t4840 = t177 * t497;
    let t4841 = t4839 * t4840;
    let t4843 = 2.0_f64 / 45.0_f64 * t161 * t4841;
    let t4844 = t1554 * t852;
    let t4845 = t161 * t4844;
    let t4846 = t4845 / 135.0_f64;
    let t4847 = t1825 * t1083;
    (t4836, t4837, t4838, t4839, t4840, t4841, t4843, t4844, t4845, t4846, t4847)
}
