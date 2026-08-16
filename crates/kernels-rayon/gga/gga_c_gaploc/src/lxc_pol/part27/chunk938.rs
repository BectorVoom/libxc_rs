//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 938/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk938(t10293: f64, t3366: f64, t605: f64, t4349: f64, t2902: f64, t921: f64, t1382: f64, t1016: f64, t2497: f64, t3381: f64, t4379: f64, t2366: f64, t2754: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10294 = 2.0_f64 * t10293;
    let t10295 = t3366 * t605;
    let t10296 = t4349 * t10295;
    let t10297 = 6.0_f64 * t10296;
    let t10298 = t2902 * t921;
    let t10299 = t1382 * t10298;
    let t10300 = 2.0_f64 * t10299;
    let t10301 = t1016 * t2497;
    let t10302 = t1382 * t10301;
    let t10303 = 2.0_f64 * t10302;
    let t10308 = t4379 * t3381;
    let t10309 = 0.14896037479937677779e-1_f64 * t10308;
    let t10310 = t2366 * t2754;
    (t10294, t10295, t10296, t10297, t10298, t10299, t10300, t10301, t10302, t10303, t10309, t10310)
}
