//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 524/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk524(t1885: f64, t3534: f64, t1820: f64, t2807: f64, t1714: f64, t3465: f64, t3469: f64, t657: f64, t3473: f64, t1688: f64, t1709: f64, t25: f64, t2696: f64, t2710: f64, t3467: f64, t3471: f64, t3475: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3535 = t1885 * t3534;
    let t3537 = 8.0_f64 / 15.0_f64 * t1820 * t3535;
    let t3538 = 8.0_f64 / 45.0_f64 * t2807;
    let t3544 = t1714 * t3465;
    let t3547 = t657 * t3469;
    let t3550 = t657 * t3473;
    let t3553 = t1688 + 0.23994444444444444444e-1_f64 * t2696 - 0.23994444444444444445e-1_f64 * t3467 + 0.71983333333333333334e-1_f64 * t3471 - 0.35991666666666666667e-1_f64 * t3475 + t1709 + 0.8888888888888888889e-2_f64 * t2710 - 0.22222222222222222222e-2_f64 * t25 * t3544 + 0.13333333333333333333e-1_f64 * t25 * t3547 - 0.66666666666666666667e-2_f64 * t25 * t3550;
    (t3535, t3537, t3538, t3544, t3547, t3550, t3553)
}
