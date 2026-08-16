//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2687/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2687(t10130: f64, t1399: f64, t14122: f64, t1437: f64, t4004: f64, t47442: f64, t47444: f64, t48438: f64, t49280: f64, t49399: f64, t49403: f64, t49407: f64, t49426: f64, t5659: f64, t5745: f64, t5755: f64, t5767: f64, t820: f64, t9899: f64) -> f64 {
    let t49428 = 0.17563392970889009434e0_f64 * t49399 + 0.58544643236296698112e-1_f64 * t49403 + t47442 + 0.29272321618148349057e-1_f64 * t49407 - 0.65854491829355115987e0_f64 * t820 * t5767 * t9899 - 0.65854491829355115987e0_f64 * t820 * t1437 * t48438 - 0.19756347548806534796e1_f64 * t5755 * t49280 * t1399 + 0.11853808529283920877e2_f64 * t5745 * t14122 * t4004 - 0.19756347548806534796e1_f64 * t820 * t10130 * t5659 + 0.91069445034239308175e-1_f64 * t47444 - 0.19514881078765566037e-2_f64 * t49426;
    t49428
}
