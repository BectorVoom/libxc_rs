//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 390/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk390(t514: f64, t633: f64, t1905: f64, t1754: f64, t1765: f64, t1684: f64, t1735: f64, t1732: f64, t1738: f64, t1762: f64, t1769: f64, t513: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t1906 = t514 * t633;
    let t1907 = t1905 * t1906;
    let t1910 = 4.0_f64 * t1754;
    let t1912 = 1.3333333333333333_f64 * t1765;
    let t1914 = 0.821419393556371_f64 * t1684;
    let t1916 = 0.2738064645187903_f64 * t1735;
    let t1918 = t1910 - 4.0_f64 * t1762 + t1912 + 4.0_f64 * t1769 + t1914 - 0.821419393556371_f64 * t1732 + t1916 + 0.821419393556371_f64 * t1738;
    let t1919 = 1.0_f64 / t513;
    (t1906, t1907, t1910, t1912, t1914, t1916, t1918, t1919)
}
