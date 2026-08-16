//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 527/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk527(t4309: f64, t486: f64, t3913: f64, t41: f64, t470: f64, t1483: f64, t1493: f64, t1497: f64, t4224: f64, t4227: f64, t4233: f64, t4238: f64, t4242: f64, t4298: f64, t4302: f64, t4307: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4310 = t486 * t4309;
    let t4312 = t3913 * t41;
    let t4313 = t4312 * t470;
    let t4314 = t486 * t4313;
    let t4316 = t1483 * t1493;
    let t4318 = t1483 * t1497;
    let t4320 = t4224 / 128.0_f64 - t4227 / 24.0_f64 + t4233 / 96.0_f64 - t4238 / 128.0_f64 - t4242 / 72.0_f64 + t4298 / 16.0_f64 - t4302 / 256.0_f64 - t4307 / 576.0_f64 - 2.0_f64 / 9.0_f64 * t4310 + 11.0_f64 / 18.0_f64 * t4314 - t4316 / 3.0_f64 + t4318 / 12.0_f64;
    (t4310, t4312, t4313, t4314, t4316, t4318, t4320)
}
