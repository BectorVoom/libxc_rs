//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 952/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk952(t219: f64, t4048: f64, t3589: f64, t2114: f64, t4564: f64, t1529: f64, t1960: f64, t1466: f64, t3667: f64, t1401: f64, t3899: f64, t3476: f64, t5146: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11913 = t4048 * t219;
    let t11914 = t11913 * t3589;
    let t11946 = t2114 * t4564;
    let t11947 = 8.0_f64 / 45.0_f64 * t11946;
    let t11954 = t1960 * t1529;
    let t11955 = 4.0_f64 / 45.0_f64 * t11954;
    let t11983 = t1466 * t3667;
    let t11989 = t3899 * t1401;
    let t12025 = t5146 * t3476;
    (t11914, t11947, t11955, t11983, t11989, t12025)
}
