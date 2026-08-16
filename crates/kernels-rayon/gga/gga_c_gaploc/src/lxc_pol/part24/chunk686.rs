//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 686/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk686(t4082: f64, t4085: f64, t6372: f64, t1250: f64, t2280: f64, t1254: f64, t864: f64, t6363: f64, t6366: f64, t6374: f64, t2287: f64, t471: f64, t64: f64, t869: f64, t90: f64) -> (f64, f64, f64, f64) {
    let t6377 = t4082 * t6372 * t4085;
    let t6379 = t2280 * t1250;
    let t6381 = t864 * t1254;
    let t6383 = 189.0_f64 / 512.0_f64 * t6363 - 483.0_f64 / 16384.0_f64 * t6366 + 147.0_f64 / 1048576.0_f64 * t6374 - 49.0_f64 / 1048576.0_f64 * t6377 + 161.0_f64 / 16384.0_f64 * t6379 - 63.0_f64 / 512.0_f64 * t6381;
    let t6393 = t6383 * t471 - 8.0_f64 / 3.0_f64 * t2287 * t64 + 4.0_f64 / 3.0_f64 * t869 * t90 + 63.0_f64 / 512.0_f64 * t6363 - 49.0_f64 / 16384.0_f64 * t6366 + 49.0_f64 / 49152.0_f64 * t6379 - 21.0_f64 / 512.0_f64 * t6381;
    (t6377, t6379, t6381, t6393)
}
