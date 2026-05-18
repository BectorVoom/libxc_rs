//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 140/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk140<F: Float>(t231: F, t242: F, t337: F, t359: F, t4: F, t55: F, t624: F, t631: F, t637: F, t638: F, t79: F, t1: F) -> (F, F) {
    let t642 = t231 * (F::new(0.53236443333333333332e-3) * t4 * t79 * t242 + F::new(1.0) * t624 * t631 - t337 - t359 + F::new(0.18311555036753159941e-3) * t4 * t79 * t55 + F::new(0.58482233974552040708e0) * t637 * t638);
    let t643 = t231 * t1;
    (t642, t643)
}
