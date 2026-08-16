//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 382/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk382(t122: f64, t569: f64, t886: f64, t486: f64, t844: f64, t350: f64, t839: f64, t1464: f64, t764: f64, t337: f64) -> (f64, f64, f64, f64, f64) {
    let t1813 = t122 * t569 * t886;
    let t1816 = t486 * t844 / 30.0_f64;
    let t1818 = t350 * t839;
    let t1820 = t1464 * t764;
    let t1821 = t1820 * t337;
    (t1813, t1816, t1818, t1820, t1821)
}
