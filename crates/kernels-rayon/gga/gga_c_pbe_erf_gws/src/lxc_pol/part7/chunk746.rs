//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 746/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk746(t6110: f64, t825: f64, t822: f64, t2418: f64, t338: f64, t892: f64, t2220: f64, t939: f64, t2359: f64, t2373: f64, t2379: f64, t2384: f64, t2388: f64, t2408: f64, t335: f64, t4385: f64, t4459: f64, t4464: f64, t4467: f64, t4469: f64, t4475: f64, t4477: f64, t4484: f64, t4487: f64, t4489: f64, t4493: f64, t4496: f64, t6107: f64, t833: f64) -> (f64, f64, f64, f64, f64) {
    let t6111 = t6110 * t825;
    let t6112 = t822 * t6111;
    let t6116 = t338 * t892 * t2418;
    let t6120 = t338 * t2220 * t939;
    let t6123 = -t2359 * t4459 / 32.0_f64 - t2359 * t4464 / 96.0_f64 + 7.0_f64 / 48.0_f64 * t4467 + 7.0_f64 / 24.0_f64 * t4469 - t2388 * t2373 / 16.0_f64 - 7.0_f64 / 96.0_f64 * t4475 - 7.0_f64 / 96.0_f64 * t4477 - t2384 * t2379 / 32.0_f64 + t4385 * t4484 / 32.0_f64 + 35.0_f64 / 144.0_f64 * t4487 + 7.0_f64 / 48.0_f64 * t4489 + t2408 * t4493 / 16.0_f64 + 7.0_f64 / 48.0_f64 * t4496 + t6107 * t833 / 96.0_f64 + t6112 * t833 / 96.0_f64 + t335 * t6116 / 16.0_f64 - t335 * t6120 / 32.0_f64;
    (t6111, t6112, t6116, t6120, t6123)
}
