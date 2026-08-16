//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 687/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk687(t13276: f64, t6320: f64, t2268: f64, t12798: f64, t12383: f64, t12386: f64, t12392: f64, t12395: f64, t12397: f64, t12400: f64, t471: f64, t3526: f64, t871: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13277 = t6320 * t13276;
    let t13279 = 0.17073003981405689759e0_f64 * t2268 * t13277;
    let t13280 = 0.47425011059460249332e-2_f64 * t12798;
    let t13287 = -3.0_f64 / 128.0_f64 * t12383 - 27.0_f64 / 4096.0_f64 * t12386 + 27.0_f64 / 262144.0_f64 * t12392 - 9.0_f64 / 262144.0_f64 * t12395 + 9.0_f64 / 4096.0_f64 * t12397 + t12400 / 128.0_f64;
    let t13288 = t13287 * t471;
    let t13289 = t3526 * t871;
    (t13277, t13279, t13280, t13287, t13288, t13289)
}
