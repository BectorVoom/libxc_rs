//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 971/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk971(t3604: f64, t4521: f64, t3589: f64, t4048: f64, t581: f64, t11753: f64, t1627: f64, t4537: f64, t1926: f64, t4204: f64, t4183: f64, t1298: f64, t4564: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t13812 = t4521 * t3604;
    let t13829 = t4048 * t581 * t3589;
    let t13846 = 0.0016792592592592592_f64 * t11753;
    let t13915 = t4537 * t1627;
    let t13916 = 0.21642082724729686_f64 * t13915;
    let t13917 = t1926 * t4204;
    let t13919 = t1926 * t4183;
    let t13924 = t1298 * t4564;
    (t13812, t13829, t13846, t13916, t13917, t13919, t13924)
}
