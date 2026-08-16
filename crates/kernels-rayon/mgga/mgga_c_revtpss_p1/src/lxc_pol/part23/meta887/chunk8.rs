//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2809/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2809(t4321: f64, t6072: f64, t689: f64, t18800: f64, t41003: f64, t41004: f64, t41034: f64, t41037: f64, t41049: f64, t4487: f64, t51199: f64, t51203: f64, t51208: f64, t61441: f64, t61448: f64, t62516: f64, t62523: f64, t62528: f64) -> f64 {
    let t76051 = t689 * t4321 * t6072;
    let t76055 = -0.29272321618148349057e-1_f64 * t61441 - t41003 + 0.17073386770573548589e-1_f64 * t41004 + 0.21951497276451705328e-1_f64 * t61448 + 0.39512695097613069592e1_f64 * t18800 * t4487 + 0.26019841438354088051e-2_f64 * t41034 + 0.32927245914677557992e-1_f64 * t62516 + 0.11708928647259339623e0_f64 * t62523 + t41037 - 0.7805952431506226415e-1_f64 * t62528 + 0.16463622957338778997e-1_f64 * t76051 + 0.19514881078765566038e-2_f64 * t51199 + t41049 + 0.13878983423218070567e-1_f64 * t51203 + t51208;
    t76055
}
