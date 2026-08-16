//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 642/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk642(t1512: f64, t436: f64, t1517: f64, t432: f64, t1504: f64, t486: f64, t1554: f64, t512: f64, t161: f64, t1499: f64, t490: f64, t1423: f64, t1427: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3149 = t1512 * t436;
    let t3151 = t432 * t1517;
    let t3153 = t486 * t1504;
    let t3155 = t1554 * t512;
    let t3156 = t161 * t3155;
    let t3158 = t1499 * t490;
    let t3165 = t1423 * t1427;
    (t3149, t3151, t3153, t3155, t3156, t3158, t3165)
}
