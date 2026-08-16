//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1071/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1071(t4702: f64, t518: f64, t577: f64, t3416: f64, t5356: f64, t1472: f64, t5371: f64, t1454: f64, t5327: f64, t1462: f64, t1325: f64, t1440: f64, t2181: f64, t3464: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12541 = t4702 * t518;
    let t12543 = 4.0_f64 / 15.0_f64 * t12541 * t577;
    let t12545 = 4.0_f64 / 5.0_f64 * t3416 * t5356;
    let t12546 = t1472 * t5371;
    let t12547 = 16.0_f64 / 15.0_f64 * t12546;
    let t12549 = 4.0_f64 / 15.0_f64 * t5327 * t1454;
    let t12551 = 4.0_f64 / 9.0_f64 * t5327 * t1462;
    let t12555 = 4.0_f64 / 15.0_f64 * t1325 * t1440 * t2181 * t3464;
    (t12543, t12545, t12547, t12549, t12551, t12555)
}
