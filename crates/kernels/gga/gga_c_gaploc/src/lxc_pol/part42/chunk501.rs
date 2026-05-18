//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 501/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk501<F: Float>(t6576: F, t9544: F, t2334: F, t2465: F, t2464: F, t587: F, t3177: F, t6985: F, t2487: F, t589: F, t2365: F, t6510: F) -> (F, F, F, F, F) {
    let t9545 = t6576 * t9544;
    let t9547 = t2465 * t2334;
    let t9548 = t2464 * t9547;
    let t9549 = t587 * t9548;
    let t9552 = t6985 * t3177;
    let t9553 = t2487 * t9552;
    let t9555 = t589 * t3177;
    let t9556 = t587 * t9555;
    let t9558 = t2365 * t6510;
    (t9545, t9549, t9553, t9556, t9558)
}
