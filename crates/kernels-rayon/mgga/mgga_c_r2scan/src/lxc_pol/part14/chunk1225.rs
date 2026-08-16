//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1225/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1225(t39995: f64, t40001: f64, t38028: f64, t38033: f64, t39992: f64, t39998: f64, t40004: f64, t40007: f64, t40011: f64, t40016: f64, t40019: f64, t40021: f64) -> f64 {
    let t41649 = 0.27944763721877274748e0_f64 * t39995;
    let t41651 = 0.27944763721877274748e0_f64 * t40001;
    let t41660 = 0.52396431978519890152e-1_f64 * t39992 + t41649 + 0.26198215989259945076e-1_f64 * t39998 + t41651 + 0.26198215989259945076e0_f64 * t40004 - 0.5200933044032561138e0_f64 * t40007 + 0.13099107994629972538e-1_f64 * t40011 + 0.47609969197673950973e-2_f64 * t38028 + 0.62295486109113302474e-1_f64 * t38033 - 0.5200933044032561138e0_f64 * t40016 + 0.86682217400542685632e-1_f64 * t40019 + 0.21951497276451705328e0_f64 * t40021;
    t41660
}
