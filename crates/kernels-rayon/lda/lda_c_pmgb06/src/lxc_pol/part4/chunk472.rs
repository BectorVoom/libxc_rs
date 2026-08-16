//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 472/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk472(t5: f64, t153: f64, t1872: f64, t137: f64, t132: f64, t460: f64, t802: f64, t332: f64, t760: f64, t1: f64, t395: f64, t44: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t6 = t5 <= zeta_threshold;
    let t1873 = t1872 * t153;
    let t1874 = t137 * t1873;
    let t1876 = t132 * t1874 / 30.0_f64;
    let t1878 = t802 * t460 / 30.0_f64;
    let t1879 = t332 * t760;
    let t1881 = t5 * t1;
    let t1885 = piecewise3(t6, 0.0_f64, 4.0_f64 * t1881 * t395 + 2.0_f64 * t1879);
    let t1886 = t1885 * t44;
    (t1873, t1874, t1876, t1878, t1879, t1881, t1886)
}
