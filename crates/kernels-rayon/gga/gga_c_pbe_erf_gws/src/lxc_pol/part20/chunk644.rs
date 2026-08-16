//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 644/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk644(t1044: f64, t2607: f64, t1621: f64, t1620: f64, t1037: f64, t2612: f64, t3354: f64, t643: f64, t642: f64, t639: f64, t1643: f64, t3351: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3512 = t2607 * t1044;
    let t3513 = t1621 * t3512;
    let t3515 = 8.0_f64 / 15.0_f64 * t1620 * t3513;
    let t3517 = 8.0_f64 / 45.0_f64 * t2612 * t1037;
    let t3518 = t643 * t3354;
    let t3519 = t642 * t3518;
    let t3521 = 4.0_f64 / 45.0_f64 * t639 * t3519;
    let t3522 = t1643 * t3351;
    (t3512, t3513, t3515, t3517, t3518, t3519, t3521, t3522)
}
