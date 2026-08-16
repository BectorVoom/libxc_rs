//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 872/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk872(t1009: f64, t1026: f64, t2986: f64, t400: f64, t8171: f64, t1027: f64, t1030: f64, t8428: f64, t1073: f64, t3007: f64, t1184: f64, t119: f64, t395: f64, t84: f64) -> (f64, f64, f64, f64, f64) {
    let t8441 = 1.0_f64 / t1026 / t1009;
    let t8445 = 12304.676425209354_f64 * t400 * t8441 * t8171 * t2986;
    let t8449 = 51.94726769812759_f64 * t400 * t1027 * t8428 * t1030;
    let t8464 = t1073 * t3007;
    let t8469 = 0.0018989760778855128_f64 * t395 * t119 * t1184 * t84;
    (t8441, t8445, t8449, t8464, t8469)
}
