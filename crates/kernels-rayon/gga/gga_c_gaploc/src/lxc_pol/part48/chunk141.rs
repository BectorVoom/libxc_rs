//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 141/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk141(t231: f64, t242: f64, t337: f64, t359: f64, t4: f64, t55: f64, t624: f64, t631: f64, t637: f64, t638: f64, t79: f64, t1: f64) -> (f64, f64) {
    let t642 = t231 * (0.53236443333333333332e-3_f64 * t4 * t79 * t242 + 1.0_f64 * t624 * t631 - t337 - t359 + 0.18311555036753159941e-3_f64 * t4 * t79 * t55 + 0.58482233974552040708e0_f64 * t637 * t638);
    let t643 = t231 * t1;
    (t642, t643)
}
