//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 619/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk619(t1836: f64, t489: f64, t161: f64, t1933: f64, t486: f64, t1835: f64, t517: f64, t1887: f64, t436: f64, t1928: f64, t432: f64, t1873: f64, t435: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4790 = t489 * t1836;
    let t4792 = 2.0_f64 / 45.0_f64 * t161 * t4790;
    let t4794 = 2.0_f64 / 45.0_f64 * t486 * t1933;
    let t4801 = t1835 * t517;
    let t4807 = 2.0_f64 / 45.0_f64 * t1887 * t436;
    let t4809 = 2.0_f64 / 45.0_f64 * t432 * t1928;
    let t4810 = t435 * t1873;
    (t4790, t4792, t4794, t4801, t4807, t4809, t4810)
}
