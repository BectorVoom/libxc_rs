//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 848/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk848(t22456: f64, t2874: f64, t1091: f64, t19571: f64, t2881: f64, t1248: f64, t19333: f64, t296: f64, t10749: f64, t15329: f64, t15420: f64, t1901: f64, t193: f64, t22398: f64, t22402: f64, t22407: f64, t22412: f64, t22416: f64, t22441: f64, t22446: f64, t22449: f64, t22454: f64, t446: f64, t89: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t22457 = t2874 * t22456;
    let t22460 = t19571 * t1091;
    let t22461 = t2881 * t22460;
    let t22464 = t19333 * t1248;
    let t22465 = t296 * t22464;
    let t22467 = 2.0_f64 / 3.0_f64 * t1901 * t22398 - t446 * t22402 / 3.0_f64 + 2.0_f64 * t446 * t22407 - 2.0_f64 * t446 * t22412 - 2.0_f64 / 3.0_f64 * t446 * t22416 + 4.0_f64 / 9.0_f64 * t15329 + t89 * t193 * t22441 / 3.0_f64 - t446 * t22446 - t446 * t22449 / 3.0_f64 - 4.0_f64 / 9.0_f64 * t15420 - t446 * t22454 + t1901 * t22457 / 3.0_f64 + t1901 * t22461 / 3.0_f64 - t10749 - t446 * t22465;
    (t22457, t22460, t22461, t22464, t22465, t22467)
}
