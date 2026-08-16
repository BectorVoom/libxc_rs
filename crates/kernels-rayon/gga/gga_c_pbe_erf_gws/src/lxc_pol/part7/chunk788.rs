//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 788/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk788(t6466: f64, t6474: f64, t6477: f64, t6482: f64, t6486: f64, t6490: f64, t6495: f64, t6497: f64, t6502: f64, t6506: f64, t6508: f64, t6511: f64, t6513: f64, t902: f64, t914: f64, t929: f64) -> f64 {
    let t6516 = t902 * t6466 / 1536.0_f64 + t902 * t6474 / 384.0_f64 - 7.0_f64 / 384.0_f64 * t6477 - t6482 - t6486 + t6490 + t6495 - t914 * t6497 / 1536.0_f64 - 7.0_f64 / 256.0_f64 * t6502 - 119.0_f64 / 1152.0_f64 * t6506 + 7.0_f64 / 384.0_f64 * t6508 - t6511 - t929 * t6513 / 768.0_f64;
    t6516
}
