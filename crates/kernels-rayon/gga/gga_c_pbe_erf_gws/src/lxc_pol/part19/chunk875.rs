//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 875/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk875(t3228: f64, t6402: f64, t6365: f64, t904: f64, t2083: f64, t3037: f64, t3165: f64, t6: f64, t1112: f64, t2079: f64, t2319: f64, t3299: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9342 = 7.0_f64 / 576.0_f64 * t6402 * t3228;
    let t9343 = t6365 * t904;
    let t9364 = t3037 * t2083;
    let t9375 = t6 * t3165;
    let t9385 = t2079 * t1112;
    let t9386 = t904 * t9385;
    let t9415 = 7.0_f64 / 1152.0_f64 * t2319 * t3299;
    (t9342, t9343, t9364, t9375, t9385, t9386, t9415)
}
