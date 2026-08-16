//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 783/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk783<F: Float>(t29976: F, t4261: F, t9074: F, t19532: F, t30136: F, t2321: F, t30334: F, t12424: F, t2312: F, t12427: F, t484: F, t29854: F) -> (F, F, F, F, F, F, F) {
    let t39671 = t9074 * t4261 * t29976;
    let t39674 = t9074 * t19532 * t30136;
    let t39677 = t9074 * t30334 * t2321;
    let t39679 = t2312 * t12424;
    let t39681 = t2312 * t12427;
    let t39695 = t484 * t12424;
    let t39717 = t9074 * t4261 * t29854;
    (t39671, t39674, t39677, t39679, t39681, t39695, t39717)
}
