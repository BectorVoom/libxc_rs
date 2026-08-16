//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 721/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk721(t4758: f64, t6446: f64, t571: f64, t2393: f64, t4804: f64, t3794: f64, t2005: f64, t34: f64, t4829: f64, t1325: f64, t1446: f64, t2397: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6447 = t4758 * t6446;
    let t6449 = 32.0_f64 / 45.0_f64 * t571 * t6447;
    let t6451 = 16.0_f64 / 45.0_f64 * t4804 * t2393;
    let t6453 = 16.0_f64 / 45.0_f64 * t3794 * t2393;
    let t6454 = t2005 * t34;
    let t6455 = t4829 * t6454;
    let t6457 = 32.0_f64 / 45.0_f64 * t1325 * t6455;
    let t6459 = 8.0_f64 / 45.0_f64 * t1446 * t2397;
    (t6447, t6449, t6451, t6453, t6454, t6455, t6457, t6459)
}
