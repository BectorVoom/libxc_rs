//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1124/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1124(t2367: f64, t4083: f64, t14072: f64, t14084: f64, t4094: f64, t840: f64, t13894: f64, t1208: f64, t2242: f64, t4090: f64, t4414: f64, t1205: f64, t6781: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t14198 = t2367 * t4083;
    let t14229 = 119.0_f64 / 3456.0_f64 * t14072;
    let t14233 = 35.0_f64 / 216.0_f64 * t14084;
    let t14283 = t840 * t4094;
    let t14295 = 119.0_f64 / 6912.0_f64 * t13894;
    let t14302 = 35.0_f64 / 432.0_f64 * t2242 * t1208;
    let t14305 = t4414 * t4090;
    let t14309 = t6781 * t1205;
    (t14198, t14229, t14233, t14283, t14295, t14302, t14305, t14309)
}
