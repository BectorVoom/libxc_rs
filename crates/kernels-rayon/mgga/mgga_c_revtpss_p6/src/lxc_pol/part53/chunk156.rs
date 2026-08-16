//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 156/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk156(t15: f64, t580: f64, t14: f64, t2: f64, t11: f64, t22: f64, t21: f64, t3: f64, t20: f64, t12: f64, t19: f64, t27: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t582 = 2.0_f64 * t15 * t580;
    let t583 = t14 * t2;
    let t584 = t11 * t583;
    let t586 = 4.0_f64 * t584 * t22;
    let t587 = t21 * t3;
    let t588 = 1.0_f64 / t587;
    let t590 = 4.0_f64 * t20 * t588;
    let t592 = t12 * t19 * t2;
    let t594 = 6.0_f64 * t592 * t27;
    (t582, t583, t586, t587, t588, t590, t594)
}
