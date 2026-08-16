//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 671/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk671(t463: f64, t3102: f64, t137: f64, t4061: f64, t135: f64, t4074: f64, t4077: f64, t4082: f64, t4085: f64, t1247: f64, t3103: f64, t12380: f64, t464: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let pi = (M_PI as f64);
    let t12385 = pi * t463;
    let t12386 = t3102 * t12385;
    let t12389 = 1.0_f64 / t137 / t4061;
    let t12390 = t135 * t12389;
    let t12392 = t12390 * t4074 * t4077;
    let t12395 = t4082 * t12390 * t4085;
    let t12397 = t1247 * t3103;
    let t12399 = t464 * t12380;
    (t12385, t12386, t12390, t12392, t12395, t12397, t12399)
}
