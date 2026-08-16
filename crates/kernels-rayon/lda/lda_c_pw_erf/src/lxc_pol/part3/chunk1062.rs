//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1062/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1062(t3476: f64, t4500: f64, t12114: f64, t4488: f64, t10015: f64, t5148: f64, t739: f64, t944: f64, t348: f64, t3965: f64, t5147: f64, t5136: f64, t945: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12439 = t4500 * t3476;
    let t12442 = 8.0_f64 / 3.0_f64 * t4488 * t12439 * t12114;
    let t12444 = 16.0_f64 / 9.0_f64 * t10015 * t5148;
    let t12445 = t739 * t944;
    let t12446 = t12445 * t348;
    let t12449 = 8.0_f64 / 9.0_f64 * t3965 * t5147 * t12446;
    let t12450 = t5136 * t945;
    (t12442, t12444, t12445, t12446, t12449, t12450)
}
