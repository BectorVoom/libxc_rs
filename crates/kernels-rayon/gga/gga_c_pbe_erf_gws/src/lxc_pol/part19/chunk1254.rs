//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1254/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1254(t1144: f64, t14191: f64, t859: f64, t14180: f64, t4386: f64, t14949: f64, t9270: f64, t53178: f64, t53198: f64, t53230: f64, t53260: f64, t53272: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t54978 = t859 * t1144 * t14191;
    let t54984 = t4386 * t1144 * t14180;
    let t54998 = 7.0_f64 / 72.0_f64 * t9270 * t14949;
    let t55005 = 7.0_f64 / 288.0_f64 * t53178;
    let t55007 = 7.0_f64 / 288.0_f64 * t53198;
    let t55022 = 7.0_f64 / 72.0_f64 * t53230;
    let t55031 = 7.0_f64 / 72.0_f64 * t53260;
    let t55036 = 7.0_f64 / 72.0_f64 * t53272;
    (t54978, t54984, t54998, t55005, t55007, t55022, t55031, t55036)
}
