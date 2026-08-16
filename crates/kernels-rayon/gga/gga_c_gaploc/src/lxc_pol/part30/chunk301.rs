//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 301/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk301(t400: f64, t109: f64, t111: f64, t112: f64, t1189: f64, t1275: f64, t1279: f64, t1286: f64, t1287: f64, t1293: f64, t1297: f64, t427: f64, t436: f64, t437: f64, t441: f64, t75: f64) -> f64 {
    let t1301 = t400 * t400;
    let t1305 = -0.43802864444444444443e-3_f64 * t109 * t1275 * t112 - 0.2e-22_f64 * t436 * t1279 * t112 - 0.26281718666666666666e-2_f64 * t109 * t427 * t441 + 0.19711288999999999999e-2_f64 * t1286 * t1287 + 0.19711288999999999999e-2_f64 * t436 * t437 * t441 + 0.39422577999999999998e-2_f64 * t109 * t111 * t1293 - 0.19711288999999999999e-2_f64 * t109 * t111 * t1297 - 4.0_f64 * t1301 - 4.0_f64 * t75 * t1189;
    t1305
}
