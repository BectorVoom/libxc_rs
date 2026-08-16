//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1106/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1106(t12929: f64, t3974: f64, t4522: f64, t10392: f64, t4508: f64, t559: f64, t2137: f64, t5041: f64, t5045: f64, t2120: f64, t3466: f64, t10039: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12937 = 16.0_f64 / 9.0_f64 * t3974 * t4522 * t12929;
    let t12941 = 32.0_f64 / 15.0_f64 * t3974 * t4508 * t559 * t10392;
    let t12942 = t5041 * t2137;
    let t12943 = 8.0_f64 / 15.0_f64 * t12942;
    let t12944 = t5045 * t2137;
    let t12945 = 16.0_f64 / 15.0_f64 * t12944;
    let t12947 = 4.0_f64 / 15.0_f64 * t2120 * t3466;
    let t12948 = 8.0_f64 / 15.0_f64 * t10039;
    (t12937, t12941, t12943, t12945, t12947, t12948)
}
