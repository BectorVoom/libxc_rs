//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 346/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk346<F: Float>(t1266: F, t1275: F, t2141: F, t1234: F, t1264: F, t2115: F, t2129: F, t2133: F, t361: F, t374: F, t45: F, t67: F) -> (F, F) {
    let t2143 = t1266 * t2141 * t1275;
    let t2146 = -F::new(0.62182e-1) * t2115 * t361 + F::new(1.0) * t1234 * t2129 + F::cast_from(0.19751789702565206229e-1_f64) * t45 * t2133 * t374 - F::cast_from(0.58482233974552040708e0_f64) * t1264 * t2143;
    let t2147 = t67 * t2146;
    (t2143, t2147)
}
