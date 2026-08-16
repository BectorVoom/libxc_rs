//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 1347/1429 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk1347(t10086: f64, t1125: f64, t2469: f64, t12282: f64, t36095: f64, t36098: f64, t36100: f64, t36103: f64, t36105: f64, t36108: f64, t36109: f64, t36111: f64, t36113: f64, t36116: f64, t36127: f64, t36130: f64, t36252: f64, t36255: f64, t36259: f64, t3846: f64, t7053: f64, t7056: f64) -> f64 {
    let t36262 = 2.0_f64 * t2469 * t1125 * t10086;
    let t36263 = 4.0_f64 * t12282 * t7056 - t3846 * t7053 + t36095 + t36098 + t36100 + t36103 - t36105 + t36108 + t36109 - t36111 + t36113 + t36116 - t36127 + t36130 + t36252 - t36255 + t36259 + t36262;
    t36263
}
