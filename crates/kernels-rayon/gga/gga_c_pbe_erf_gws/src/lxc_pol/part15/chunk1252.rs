//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1252/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1252(t3038: f64, t3972: f64, t3975: f64, t9520: f64, t14643: f64, t840: f64, t14793: f64, t9270: f64, t1144: f64, t13909: f64, t859: f64, t1176: f64, t14639: f64, t6365: f64, t923: f64) -> (f64, f64, f64, f64, f64) {
    let t53395 = t3972 * t3975 * t3038 * t9520;
    let t53405 = 7.0_f64 / 144.0_f64 * t840 * t14643;
    let t53407 = 7.0_f64 / 24.0_f64 * t9270 * t14793;
    let t53419 = t859 * t1144 * t13909;
    let t53424 = t1176 * t923 * t6365 * t14639;
    (t53395, t53405, t53407, t53419, t53424)
}
