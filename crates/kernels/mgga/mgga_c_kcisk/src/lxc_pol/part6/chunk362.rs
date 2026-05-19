//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 362/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk362<F: Float>(t1529: F, t1542: F, t1550: F, t2081: F, t2095: F, t2098: F, t2107: F, t2285: F, t2293: F, t2297: F, t240: F, t516: F) -> F {
    let t2306 = -t2081 + t2095 + t240 * (-F::new(0.3109e-1) * t2285 * t516 + F::new(1.0) * t1529 * t2293 + t2081 - t2095 - F::cast_from(0.19751789702565206229e-1_f64) * t2098 + F::cast_from(0.58482233974552040708e0_f64) * t1542 * t2297) + F::cast_from(0.19751789702565206229e-1_f64) * t240 * t2098 - F::cast_from(0.58482233974552040708e0_f64) * t1550 * t2107;
    t2306
}
