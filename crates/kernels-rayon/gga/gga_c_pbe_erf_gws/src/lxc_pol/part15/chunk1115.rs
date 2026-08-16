//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1115/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1115(t14079: f64, t918: f64, t1477: f64, t326: f64, t346: f64, t1185: f64, t2339: f64, t4039: f64, t2273: f64, t2278: f64, t850: f64, t852: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t14080 = t14079 * t918;
    let t14081 = 7.0_f64 / 576.0_f64 * t14080;
    let t14083 = t326 * t346 * t1477;
    let t14084 = t14083 * t1185;
    let t14085 = 35.0_f64 / 432.0_f64 * t14084;
    let t14086 = t4039 * t2339;
    let t14088 = t4039 * t2273;
    let t14091 = t850 * t2278 * t852;
    (t14080, t14081, t14083, t14085, t14086, t14088, t14091)
}
