//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 911/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk911(t43660: f64, t43679: f64, t43681: f64, t11848: f64, t2021: f64, t7372: f64, t11576: f64, t123: f64, t883: f64, t2684: f64, t2685: f64, t11608: f64, t2464: f64, t2465: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t45457 = 0.34082600847929555269e0_f64 * t43660;
    let t45458 = 0.59584149919750711116e-1_f64 * t43679;
    let t45459 = 0.71500979903700853339e0_f64 * t43681;
    let t45463 = t2021 * t11848 * t7372;
    let t45464 = 0.14896037479937677779e-1_f64 * t45463;
    let t45466 = t11576 * t123 * t883;
    let t45468 = t2684 * t2685 * t45466;
    let t45469 = 0.19171462976960374838e0_f64 * t45468;
    let t45472 = t2684 * t2464 * t2465 * t11608;
    (t45457, t45458, t45459, t45464, t45466, t45469, t45472)
}
