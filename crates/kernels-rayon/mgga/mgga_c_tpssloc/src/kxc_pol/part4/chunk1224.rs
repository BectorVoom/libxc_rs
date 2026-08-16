//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 1224/1228 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk1224(t6287: f64, t671: f64, t1774: f64, t4072: f64, t1266: f64, t5493: f64, t1271: f64, t1393: f64, t1459: f64, t19450: f64, t19451: f64, t19456: f64, t19461: f64, t2314: f64, t4028: f64, t4034: f64, t4037: f64, t510: f64, t5450: f64, t5457: f64, t5494: f64, t6295: f64, t6468: f64, t650: f64, t652: f64, t672: f64) -> f64 {
    let t20127 = t6287 * t671;
    let t20136 = t1774 * t4072;
    let t20143 = t1266 * t5493;
    let t20147 = -t1266 * t5450 - 2.0_f64 * t1266 * t5457 + t1271 * t6468 + t1393 * t6295 - 4.0_f64 * t1459 * t19456 - t19450 * t510 - 2.0_f64 * t19451 * t672 - 2.0_f64 * t19461 * t510 - 2.0_f64 * t20127 * t652 - 4.0_f64 * t20136 * t652 - 2.0_f64 * t20143 * t652 - 2.0_f64 * t2314 * t5494 - 4.0_f64 * t4028 * t4037 - 2.0_f64 * t4034 * t5494 - t6287 * t650;
    t20147
}
