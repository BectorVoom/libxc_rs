//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3186/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3186(t1222: f64, t140: f64, t24830: f64, t12840: f64, t17429: f64, t17747: f64, t20795: f64, t21020: f64, t21134: f64, t24573: f64, t3626: f64, t5308: f64, t5312: f64, t5331: f64, t5373: f64, t59355: f64, t6611: f64, t70914: f64, t70942: f64, t81177: f64, t81198: f64, t81202: f64, t82293: f64) -> f64 {
    let t83699 = t1222 * t140 * t24830;
    let t83712 = 0.42874018118069736972e-3_f64 * t17429 * t24573 + 0.42874018118069736972e-3_f64 * t5331 * t3626 * t20795 * t21020 + 0.85748036236139473947e-3_f64 * t17747 * t3626 * t82293 * t12840 - 0.85748036236139473944e-3_f64 * t70914 - 0.68598428988911579154e-2_f64 * t59355 * t6611 - 11.0_f64 / 162.0_f64 * t70942 + t83699 / 108.0_f64 - t1222 * t5308 * t81202 / 48.0_f64 - t1222 * t5308 * t81198 / 48.0_f64 + t1222 * t5312 * t81177 / 72.0_f64 + t5373 * t21134 / 18.0_f64;
    t83712
}
