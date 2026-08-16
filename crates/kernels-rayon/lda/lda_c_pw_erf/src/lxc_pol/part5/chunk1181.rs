//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1181/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1181(t12025: f64, t20737: f64, t4488: f64, t1401: f64, t7456: f64, t1466: f64, t571: f64, t593: f64, t2163: f64, t6205: f64, t4763: f64, t6993: f64) -> (f64, f64, f64, f64) {
    let t21489 = 8.0_f64 / 3.0_f64 * t4488 * t12025 * t20737;
    let t21490 = t1401 * t7456;
    let t21494 = 4.0_f64 / 15.0_f64 * t571 * t1466 * t21490 * t593;
    let t21496 = 4.0_f64 / 5.0_f64 * t6205 * t2163;
    let t21498 = 4.0_f64 / 5.0_f64 * t4763 * t6993;
    (t21489, t21494, t21496, t21498)
}
