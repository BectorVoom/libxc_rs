//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1224/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1224(t20366: f64, t20370: f64, t20377: f64, t20381: f64, t20385: f64, t20392: f64, t20395: f64, t20400: f64, t20410: f64, t20414: f64, t20424: f64, t20428: f64, t20431: f64, t20435: f64, t20437: f64, t20453: f64, t20459: f64, t20468: f64, t20489: f64, t20493: f64, t20499: f64, t20511: f64) -> (f64, f64) {
    let t21689 = -t20366 - t20370 + t20377 + t20381 + t20385 + t20392 - t20395 - t20400 + t20410 + t20414 + t20424;
    let t21690 = t20428 - t20431 - t20435 - t20437 - t20453 - t20459 + t20468 - t20489 + t20493 + t20499 + t20511;
    (t21689, t21690)
}
