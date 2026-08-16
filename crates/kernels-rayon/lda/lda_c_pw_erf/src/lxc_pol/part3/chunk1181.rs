//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1181/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1181(t3416: f64, t4943: f64, t1627: f64, t4537: f64, t1926: f64, t4204: f64, t4183: f64, t185: f64, t4567: f64, t4723: f64, t1298: f64, t4564: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13914 = 8.0_f64 / 5.0_f64 * t3416 * t4943;
    let t13915 = t4537 * t1627;
    let t13916 = 0.21642082724729686_f64 * t13915;
    let t13917 = t1926 * t4204;
    let t13919 = t1926 * t4183;
    let t13922 = t185 * t4567 * t4723;
    let t13923 = 32.0_f64 / 45.0_f64 * t13922;
    let t13924 = t1298 * t4564;
    (t13914, t13916, t13917, t13919, t13923, t13924)
}
