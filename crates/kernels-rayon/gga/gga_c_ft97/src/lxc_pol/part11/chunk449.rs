//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 449/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk449(t238: f64, t2455: f64, t695: f64, t224: f64, t2381: f64, t2384: f64, t2387: f64, t2389: f64, t2396: f64, t2419: f64, t2422: f64, t2429: f64, t678: f64) -> f64 {
    let t239 = 0.1e-59_f64 < t238;
    let t2456 = t695 * t2455;
    let t2459 = piecewise3(t239, 0.67598802253579164263e-4_f64 * t2381 * t2384 + 0.23254900946437792e-1_f64 * t2387 * t2389 + 0.23254900946437792e-2_f64 * t678 * t2396 - 0.11627450473218896e-1_f64 * t678 * t2419 + 0.19365723406274399941e-3_f64 * t678 * t2422 + 2.0_f64 * t224 * t2429 - t224 * t2456, 0.0_f64);
    t2459
}
