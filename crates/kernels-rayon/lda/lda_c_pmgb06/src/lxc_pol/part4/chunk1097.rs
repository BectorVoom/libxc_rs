//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1097/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1097(t1977: f64, t3226: f64, t1447: f64, t4605: f64, t2012: f64, t431: f64, t5210: f64, t1423: f64, t5171: f64, t1512: f64, t1928: f64, t432: f64, t4810: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13213 = t3226 * t1977;
    let t13215 = t1447 * t4605;
    let t13218 = t431 * t5210 * t2012;
    let t13220 = t1423 * t5171;
    let t13230 = t1512 * t1928;
    let t13232 = t432 * t4810;
    (t13213, t13215, t13218, t13220, t13230, t13232)
}
