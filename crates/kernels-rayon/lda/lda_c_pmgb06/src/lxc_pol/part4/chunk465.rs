//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 465/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk465(t12: f64, t176: f64, t1835: f64, t166: f64, t161: f64, t337: f64, t764: f64, t1: f64, t395: f64, t44: f64, t131: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t13 = t12 <= zeta_threshold;
    let t1836 = t1835 * t176;
    let t1837 = t166 * t1836;
    let t1839 = t161 * t1837 / 30.0_f64;
    let t1840 = t337 * t764;
    let t1842 = t12 * t1;
    let t1846 = piecewise3(t13, 0.0_f64, -4.0_f64 * t1842 * t395 + 2.0_f64 * t1840);
    let t1847 = t1846 * t44;
    let t1848 = t1847 * t131;
    (t1836, t1837, t1839, t1840, t1842, t1847, t1848)
}
