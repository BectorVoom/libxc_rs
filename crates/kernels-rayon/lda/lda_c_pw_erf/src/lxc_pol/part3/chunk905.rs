//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 905/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk905(t211: f64, t3670: f64, t514: f64, t218: f64, t3666: f64, t1513: f64, t1519: f64, t3437: f64, t565: f64, t198: f64, t4567: f64, t185: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9234 = t211 * t514 * t3670;
    let t9237 = 1.0_f64 / t3666 / t218;
    let t9244 = t1513 * t1519;
    let t9246 = t565 * t3437;
    let t9248 = t4567 * t198;
    let t9250 = 112.0_f64 / 1215.0_f64 * t185 * t9248;
    (t9234, t9237, t9244, t9246, t9248, t9250)
}
