//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1067/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1067(t1114: f64, t4409: f64, t2362: f64, t2388: f64, t2392: f64, t2408: f64, t3052: f64, t3055: f64, t3066: f64, t335: f64, t4459: f64, t4464: f64, t6778: f64, t6833: f64, t827: f64, t9323: f64, t9328: f64, t9691: f64, t9695: f64, t9697: f64, t9701: f64, t9704: f64, t9709: f64, t9718: f64, t9723: f64, t9726: f64) -> f64 {
    let t9729 = t1114 * t4409;
    let t9737 = t3066 * t9323 / 48.0_f64 + t2408 * t9328 / 48.0_f64 - t335 * t9691 / 96.0_f64 + t9695 + t9697 * t6778 / 32.0_f64 - t9701 + t2408 * t9704 / 24.0_f64 - t827 * t9709 / 48.0_f64 - t2388 * t3052 / 48.0_f64 - t2392 * t3052 / 48.0_f64 - t827 * t9718 / 24.0_f64 - t827 * t9723 / 24.0_f64 - t9726 * t2362 / 48.0_f64 - t9729 * t2362 / 48.0_f64 - t3055 * t4459 / 48.0_f64 - t3055 * t4464 / 96.0_f64 - 7.0_f64 / 48.0_f64 * t6833;
    t9737
}
