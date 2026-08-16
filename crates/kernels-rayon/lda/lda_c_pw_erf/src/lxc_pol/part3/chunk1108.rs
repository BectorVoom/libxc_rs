//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1108/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1108(t34: f64, t3966: f64, t12475: f64, t1314: f64, t12937: f64, t12941: f64, t12943: f64, t12945: f64, t12947: f64, t12948: f64, t12949: f64, t12950: f64, t12952: f64, t12955: f64, t12959: f64, t12962: f64) -> (f64, f64) {
    let t12963 = t3966 * t34;
    let t12966 = 16.0_f64 / 15.0_f64 * t12475 * t12963 * t1314;
    let t12967 = -t12937 + t12941 + t12943 + t12945 + t12947 + t12948 + t12949 + t12950 + t12952 + t12955 + t12959 + t12962 - t12966;
    (t12966, t12967)
}
