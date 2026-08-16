//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1112/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1112(t1447: f64, t4728: f64, t4732: f64, t5442: f64, t5499: f64, t1916: f64, t3226: f64, t5448: f64, t1894: f64, t3220: f64, t1898: f64, t1902: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13885 = t1447 * t4728;
    let t13887 = t1447 * t4732;
    let t13891 = t5499 * t5442;
    let t13893 = t3226 * t1916;
    let t13895 = t1447 * t5448;
    let t13905 = t3220 * t1894;
    let t13907 = t3220 * t1898;
    let t13909 = t3220 * t1902;
    (t13885, t13887, t13891, t13893, t13895, t13905, t13907, t13909)
}
