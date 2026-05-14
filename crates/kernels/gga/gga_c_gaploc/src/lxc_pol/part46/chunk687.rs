//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 687/884 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk687<F: Float>(t19532: F, t30136: F, t9074: F, t2321: F, t30334: F, t12424: F, t2312: F, t12427: F, t484: F, t29854: F, t4261: F, t30301: F, t12360: F, t12352: F, t2317: F, t6525: F, t9061: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t39674 = t9074 * t19532 * t30136;
    let t39677 = t9074 * t30334 * t2321;
    let t39679 = t2312 * t12424;
    let t39681 = t2312 * t12427;
    let t39695 = t484 * t12424;
    let t39717 = t9074 * t4261 * t29854;
    let t39731 = t9074 * t30301 * t2321;
    let t39764 = t484 * t12360;
    let t39766 = t484 * t12352;
    let t39774 = t6525 * t9061 * t2317;
    (t39674, t39677, t39679, t39681, t39695, t39717, t39731, t39764, t39766, t39774)
}
