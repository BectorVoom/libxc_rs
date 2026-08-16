//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2708/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2708(t1437: f64, t19445: f64, t20201: f64, t20204: f64, t20288: f64, t2235: f64, t2240: f64, t39054: f64, t39063: f64, t3953: f64, t4021: f64, t5389: f64, t5445: f64, t605: f64, t645: f64, t75356: f64, t75392: f64, t75419: f64, t75547: f64, t9231: f64, t9239: f64) -> f64 {
    let t75552 = -12.0_f64 * t3953 * t19445 - 120.0_f64 * t39054 * t20201 + 840.0_f64 * t39063 * t20201 * t645 - 360.0_f64 * t9239 * t5389 * t4021 + 60.0_f64 * t9231 * t20204 - 360.0_f64 * t9239 * t20204 * t645 + 60.0_f64 * t2240 * t4021 * t5445 + 60.0_f64 * t2240 * t1437 * t19445 - 4.0_f64 * t2235 * t20288 + 20.0_f64 * t2240 * t20288 * t645 - 4.0_f64 * t605 * (t75356 + t75392 + t75419 + t75547);
    t75552
}
