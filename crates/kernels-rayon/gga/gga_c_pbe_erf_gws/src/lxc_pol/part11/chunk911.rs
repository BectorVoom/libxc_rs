//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 911/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk911(t1447: f64, t156: f64, t4782: f64, t4788: f64, t1396: f64, t542: f64, t1392: f64, t4749: f64, t1285: f64, t1290: f64, t1293: f64, t395: f64, t403: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t18594 = 0.38024868119570572865e2_f64 * t1447 * t156 * t4782;
    let t18599 = 0.21687161765563048428e-1_f64 * t1447 * t156 * t4788;
    let t18604 = 0.43374323531126096856e-1_f64 * t1447 * t542 * t1396;
    let t18607 = 0.1284251895870376528e1_f64 * t1447 * t542 * t1392;
    let t18610 = 0.38527556876111295841e1_f64 * t1447 * t156 * t4749;
    let t18619 = 0.34366858576436911004e1_f64 * t395 * t1290 * t1285 * t1293 * t403;
    (t18594, t18599, t18604, t18607, t18610, t18619)
}
