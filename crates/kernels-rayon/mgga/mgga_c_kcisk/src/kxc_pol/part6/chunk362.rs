//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 362/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk362(t1529: f64, t1542: f64, t1550: f64, t2081: f64, t2095: f64, t2098: f64, t2107: f64, t2285: f64, t2293: f64, t2297: f64, t240: f64, t516: f64) -> f64 {
    let t2306 = -t2081 + t2095 + t240 * (-0.3109e-1_f64 * t2285 * t516 + 1.0_f64 * t1529 * t2293 + t2081 - t2095 - 0.19751789702565206229e-1_f64 * t2098 + 0.58482233974552040708e0_f64 * t1542 * t2297) + 0.19751789702565206229e-1_f64 * t240 * t2098 - 0.58482233974552040708e0_f64 * t1550 * t2107;
    t2306
}
