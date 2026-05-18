//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 784/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk784<F: Float>(t2321: F, t30301: F, t9074: F, t12360: F, t484: F, t12352: F, t2317: F, t6525: F, t9061: F, t30019: F, t883: F, t2300: F) -> (F, F, F, F, F, F) {
    let t39731 = t9074 * t30301 * t2321;
    let t39764 = t484 * t12360;
    let t39766 = t484 * t12352;
    let t39774 = t6525 * t9061 * t2317;
    let t39776 = t883 * t30019;
    let t39778 = t9074 * t2300 * t39776;
    (t39731, t39764, t39766, t39774, t39776, t39778)
}
