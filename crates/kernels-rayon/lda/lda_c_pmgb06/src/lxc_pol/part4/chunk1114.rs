//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1114/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1114(t1423: f64, t5365: f64, t486: f64, t5102: f64, t1499: f64, t2018: f64, t464: f64, t4680: f64, t5350: f64, t3453: f64, t831: f64, t3055: f64, t802: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t13950 = t1423 * t5365;
    let t13971 = t486 * t5102;
    let t13973 = t1499 * t2018;
    let t13979 = t4680 * t464;
    let t13984 = t1423 * t5350;
    let t14011 = t831 * t3453;
    let t14015 = t802 * t3055;
    (t13950, t13971, t13973, t13979, t13984, t14011, t14015)
}
