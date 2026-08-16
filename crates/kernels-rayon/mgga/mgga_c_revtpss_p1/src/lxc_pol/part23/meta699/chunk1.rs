//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2449/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2449(t1340: f64, t40165: f64, t268: f64, t520: f64, t39768: f64, t190: f64, t22: f64, t519: f64, t39762: f64, t1317: f64, t9545: f64, t40129: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t47059 = 0.12304822629859687989e5_f64 * t1340 * t40165;
    let t47065 = t520 * t268;
    let t47067 = 0.19263893255070628431e1_f64 * t47065 * t39768;
    let t47070 = 24.0_f64 * t22 * t519 * t190;
    let t47072 = 0.1301229756036208781e0_f64 * t47065 * t39762;
    let t47073 = t1317 * t9545;
    let t47076 = 0.21053605041484726346e2_f64 * t1340 * t40129;
    (t47059, t47067, t47070, t47072, t47073, t47076)
}
