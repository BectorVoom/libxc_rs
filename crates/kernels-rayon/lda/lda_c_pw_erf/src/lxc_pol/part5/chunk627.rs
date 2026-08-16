//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 627/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk627(t1: f64, t1904: f64, t3: f64, t604: f64, t1635: f64, t1926: f64, t1627: f64, t20: f64, t2259: f64, t1639: f64, t1518: f64, t834: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4537 = t1904 * t1 * t3;
    let t4539 = 0.21642082724729686_f64 * t4537 * t604;
    let t4540 = t1926 * t1635;
    let t4544 = t1926 * t1627;
    let t4546 = t2259 * t20;
    let t4547 = t4546 * t1639;
    let t4561 = t1518 * t834;
    (t4537, t4539, t4540, t4544, t4546, t4547, t4561)
}
