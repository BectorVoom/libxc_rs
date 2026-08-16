//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 431/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk431(t199: f64, t718: f64, t1329: f64, t391: f64, t566: f64, t31: f64, t740: f64) -> (f64, f64, f64, f64) {
    let t1658 = 0.1675256410710088_f64 * t718 * t199;
    let t1659 = t1329 * t199;
    let t1661 = t391 * t566;
    let t1669 = t31 * t740;
    (t1658, t1659, t1661, t1669)
}
