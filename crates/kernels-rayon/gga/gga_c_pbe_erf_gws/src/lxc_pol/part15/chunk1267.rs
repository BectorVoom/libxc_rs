//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1267/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1267(t2416: f64, t4182: f64, t353: f64, t859: f64, t938: f64, t13917: f64, t14424: f64, t9551: f64, t14415: f64, t51563: f64, t14397: f64, t2367: f64) -> (f64, f64, f64, f64, f64) {
    let t53614 = t2416 * t4182;
    let t53617 = t859 * t353 * t53614 * t938;
    let t53623 = t13917 * t14424 * t9551;
    let t53625 = t51563 * t14415;
    let t53626 = 7.0_f64 / 1152.0_f64 * t53625;
    let t53629 = 7.0_f64 / 144.0_f64 * t2367 * t14397;
    (t53614, t53617, t53623, t53626, t53629)
}
