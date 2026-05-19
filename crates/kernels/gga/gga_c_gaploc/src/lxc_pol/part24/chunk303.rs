//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 303/1439 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk303<F: Float>(t400: F, t109: F, t111: F, t112: F, t1189: F, t1275: F, t1279: F, t1286: F, t1287: F, t1293: F, t1297: F, t427: F, t436: F, t437: F, t441: F, t75: F) -> F {
    let t1301 = t400 * t400;
    let t1305 = -F::cast_from(0.43802864444444444443e-3_f64) * t109 * t1275 * t112 - F::new(0.2e-22) * t436 * t1279 * t112 - F::cast_from(0.26281718666666666666e-2_f64) * t109 * t427 * t441 + F::cast_from(0.19711288999999999999e-2_f64) * t1286 * t1287 + F::cast_from(0.19711288999999999999e-2_f64) * t436 * t437 * t441 + F::cast_from(0.39422577999999999998e-2_f64) * t109 * t111 * t1293 - F::cast_from(0.19711288999999999999e-2_f64) * t109 * t111 * t1297 - F::new(4.0) * t1301 - F::new(4.0) * t75 * t1189;
    t1305
}
