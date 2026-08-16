//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 930/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk930(t10426: f64, t1820: f64, t3429: f64, t562: f64, t1821: f64, t610: f64, t1827: f64, t587: f64, t1764: f64, t3346: f64, t418: f64, t1663: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10428 = 8.0_f64 / 45.0_f64 * t1820 * t10426;
    let t10429 = t3429 * t562;
    let t10430 = t1821 * t10429;
    let t10432 = 8.0_f64 / 45.0_f64 * t1820 * t10430;
    let t10433 = t3429 * t610;
    let t10434 = t1827 * t10433;
    let t10436 = 4.0_f64 / 45.0_f64 * t587 * t10434;
    let t10437 = t1764 * t3346;
    let t10438 = t10437 * t418;
    let t10439 = t1821 * t10438;
    let t10441 = 8.0_f64 / 45.0_f64 * t587 * t10439;
    let t10442 = t1663 * t3346;
    (t10428, t10432, t10436, t10438, t10441, t10442)
}
