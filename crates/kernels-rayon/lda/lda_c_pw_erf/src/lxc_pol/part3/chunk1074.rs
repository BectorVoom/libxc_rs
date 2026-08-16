//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1074/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1074(t4753: f64, t5363: f64, t5367: f64, t1472: f64, t4930: f64, t1403: f64, t1466: f64, t2065: f64, t3667: f64, t571: f64, t10505: f64, t799: f64) -> (f64, f64, f64, f64, f64) {
    let t12577 = t4753 * t5363;
    let t12578 = 16.0_f64 / 15.0_f64 * t12577;
    let t12580 = 4.0_f64 / 5.0_f64 * t4753 * t5367;
    let t12582 = 12.0_f64 / 5.0_f64 * t1472 * t4930;
    let t12587 = 12.0_f64 / 5.0_f64 * t571 * t1466 * t3667 * t2065 * t1403;
    let t12589 = 8.0_f64 / 15.0_f64 * t10505 * t799;
    (t12578, t12580, t12582, t12587, t12589)
}
