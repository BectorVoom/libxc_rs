//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2667/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2667(t11277: f64, t19826: f64, t16163: f64, t4879: f64, t1063: f64, t19681: f64, t3172: f64, t11710: f64, t19625: f64, t4899: f64, t19687: f64, t15772: f64, t4834: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t65618 = t11277 * t19826;
    let t65627 = t4879 * t16163;
    let t65630 = t1063 * t3172 * t19681;
    let t65637 = t4899 * t11710 * t19625;
    let t65650 = t1063 * t3172 * t19687;
    let t65689 = t4834 * t15772;
    (t65618, t65627, t65630, t65637, t65650, t65689)
}
