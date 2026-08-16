//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 945/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk945(t10295: f64, t4349: f64, t2902: f64, t921: f64, t1382: f64, t1016: f64, t2497: f64, t1377: f64, t3418: f64, t605: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10296 = t4349 * t10295;
    let t10297 = 6.0_f64 * t10296;
    let t10298 = t2902 * t921;
    let t10299 = t1382 * t10298;
    let t10300 = 2.0_f64 * t10299;
    let t10301 = t1016 * t2497;
    let t10302 = t1382 * t10301;
    let t10303 = 2.0_f64 * t10302;
    let t10304 = t1377 * t3418;
    let t10305 = t3418 * t605;
    let t10306 = t1382 * t10305;
    let t10307 = 2.0_f64 * t10306;
    (t10296, t10297, t10298, t10299, t10300, t10301, t10302, t10303, t10304, t10305, t10306, t10307)
}
