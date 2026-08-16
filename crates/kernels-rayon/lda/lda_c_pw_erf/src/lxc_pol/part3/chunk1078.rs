//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1078/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1078(t3794: f64, t5378: f64, t5382: f64, t1336: f64, t5334: f64, t2146: f64, t3716: f64, t1472: f64, t4901: f64, t2143: f64, t3709: f64, t1446: f64, t4907: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t12621 = t3794 * t5378;
    let t12622 = 16.0_f64 / 15.0_f64 * t12621;
    let t12624 = 4.0_f64 / 5.0_f64 * t3794 * t5382;
    let t12626 = 8.0_f64 / 15.0_f64 * t5334 * t1336;
    let t12628 = 8.0_f64 / 9.0_f64 * t2146 * t3716;
    let t12629 = t1472 * t4901;
    let t12630 = 8.0_f64 / 9.0_f64 * t12629;
    let t12631 = t3709 * t2143;
    let t12632 = 8.0_f64 / 45.0_f64 * t12631;
    let t12633 = t1446 * t4907;
    (t12622, t12624, t12626, t12628, t12630, t12632, t12633)
}
