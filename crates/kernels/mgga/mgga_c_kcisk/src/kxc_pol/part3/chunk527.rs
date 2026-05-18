//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 527/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk527<F: Float>(t4309: F, t486: F, t3913: F, t41: F, t470: F, t1483: F, t1493: F, t1497: F, t4224: F, t4227: F, t4233: F, t4238: F, t4242: F, t4298: F, t4302: F, t4307: F) -> (F, F, F, F, F, F, F) {
    let t4310 = t486 * t4309;
    let t4312 = t3913 * t41;
    let t4313 = t4312 * t470;
    let t4314 = t486 * t4313;
    let t4316 = t1483 * t1493;
    let t4318 = t1483 * t1497;
    let t4320 = t4224 / F::new(128.0) - t4227 / F::new(24.0) + t4233 / F::new(96.0) - t4238 / F::new(128.0) - t4242 / F::new(72.0) + t4298 / F::new(16.0) - t4302 / F::new(256.0) - t4307 / F::new(576.0) - F::new(2.0) / F::new(9.0) * t4310 + F::new(11.0) / F::new(18.0) * t4314 - t4316 / F::new(3.0) + t4318 / F::new(12.0);
    (t4310, t4312, t4313, t4314, t4316, t4318, t4320)
}
