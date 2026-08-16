//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1230/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1230(t13792: f64, t52993: f64, t3089: f64, t4386: f64, t1162: f64, t14682: f64, t2158: f64, t3989: f64, t13972: f64, t14443: f64, t1123: f64, t52033: f64, t833: f64, t850: f64) -> (f64, f64, f64, f64, f64) {
    let t52994 = t13792 * t52993;
    let t52996 = t4386 * t3089;
    let t52997 = t13792 * t52996;
    let t53009 = t3989 * t14682 * t1162 * t2158;
    let t53011 = t13972 * t14443;
    let t53012 = 7.0_f64 / 2304.0_f64 * t53011;
    let t53015 = t850 * t1123 * t52033 * t833;
    (t52994, t52997, t53009, t53012, t53015)
}
