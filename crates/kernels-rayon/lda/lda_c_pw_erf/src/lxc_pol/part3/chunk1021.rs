//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1021/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1021(t9276: f64, t9280: f64, t9306: f64, t9315: f64, t9318: f64, t9338: f64, t9340: f64, t3416: f64, t5272: f64, t1318: f64, t2065: f64, t5269: f64, t549: f64, t593: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11960 = 16.0_f64 / 27.0_f64 * t9276;
    let t11961 = 8.0_f64 / 27.0_f64 * t9280;
    let t11962 = 16.0_f64 / 45.0_f64 * t9306;
    let t11963 = 16.0_f64 / 135.0_f64 * t9315;
    let t11964 = 8.0_f64 / 45.0_f64 * t9318;
    let t11965 = 16.0_f64 / 45.0_f64 * t9338;
    let t11966 = 32.0_f64 / 45.0_f64 * t9340;
    let t11968 = 16.0_f64 / 5.0_f64 * t3416 * t5272;
    let t11973 = 16.0_f64 / 5.0_f64 * t1318 * t5269 * t2065 * t549 * t593;
    (t11960, t11961, t11962, t11963, t11964, t11965, t11966, t11968, t11973)
}
