//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 620/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk620(t132: f64, t4810: f64, t1517: f64, t802: f64, t1872: f64, t464: f64, t1547: f64, t823: f64, t1554: f64, t852: f64, t161: f64, t1: f64, t1414: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4812 = 2.0_f64 / 45.0_f64 * t132 * t4810;
    let t4814 = 2.0_f64 / 45.0_f64 * t802 * t1517;
    let t4815 = t1872 * t464;
    let t4836 = t1547 * t823;
    let t4837 = t132 * t4836;
    let t4844 = t1554 * t852;
    let t4845 = t161 * t4844;
    let t4851 = t1414 * t1;
    (t4812, t4814, t4815, t4836, t4837, t4844, t4845, t4851)
}
