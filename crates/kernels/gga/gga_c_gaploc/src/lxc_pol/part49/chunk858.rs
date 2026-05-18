//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 858/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk858<F: Float>(t12399: F, t467: F, t871: F, t9117: F, t2287: F, t3113: F, t29976: F, t4261: F, t9074: F, t19532: F, t30136: F, t2321: F, t30334: F) -> (F, F, F, F, F, F) {
    let t39650 = t12399 * t467;
    let t39656 = t9117 * t871;
    let t39657 = t2287 * t3113;
    let t39671 = t9074 * t4261 * t29976;
    let t39674 = t9074 * t19532 * t30136;
    let t39677 = t9074 * t30334 * t2321;
    (t39650, t39656, t39657, t39671, t39674, t39677)
}
