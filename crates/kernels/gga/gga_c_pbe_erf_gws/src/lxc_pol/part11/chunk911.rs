//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 911/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk911<F: Float>(t1447: F, t156: F, t4782: F, t4788: F, t1396: F, t542: F, t1392: F, t4749: F, t1285: F, t1290: F, t1293: F, t395: F, t403: F) -> (F, F, F, F, F, F) {
    let t18594 = F::cast_from(0.38024868119570572865e2_f64) * t1447 * t156 * t4782;
    let t18599 = F::cast_from(0.21687161765563048428e-1_f64) * t1447 * t156 * t4788;
    let t18604 = F::cast_from(0.43374323531126096856e-1_f64) * t1447 * t542 * t1396;
    let t18607 = F::cast_from(0.1284251895870376528e1_f64) * t1447 * t542 * t1392;
    let t18610 = F::cast_from(0.38527556876111295841e1_f64) * t1447 * t156 * t4749;
    let t18619 = F::cast_from(0.34366858576436911004e1_f64) * t395 * t1290 * t1285 * t1293 * t403;
    (t18594, t18599, t18604, t18607, t18610, t18619)
}
