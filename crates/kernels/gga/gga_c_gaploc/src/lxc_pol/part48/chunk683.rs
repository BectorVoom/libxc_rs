//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 683/861 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk683<F: Float>(t1246: F, t135: F, t4074: F, t458: F, t9105: F, t1234: F, t3096: F, t1233: F, t18091: F, t18089: F, t18096: F, t92: F, t4082: F, t4085: F, t1247: F, t2282: F) -> (F, F, F, F, F, F, F, F) {
    let t39632 = t9105 * t4074 * M_PI * t1246 * t135 * t458;
    let t39635 = 1.0 / t1234 / t3096;
    let t39636 = t1233 * t39635;
    let t39637 = t39636 * t18091;
    let t39642 = t18096 * t1233 * t39635 * t18089 * t92;
    let t39644 = t135 * t9105;
    let t39646 = t4082 * t39644 * t4085;
    let t39648 = t1247 * t2282;
    (t39632, t39635, t39636, t39637, t39642, t39644, t39646, t39648)
}
