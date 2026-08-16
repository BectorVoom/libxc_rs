//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 787/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk787<F: Float>(t10256: F, t30830: F, t913: F, t2482: F, t3358: F, t9263: F, t12957: F, t31356: F, t35216: F, t9287: F, t2792: F, t3177: F) -> (F, F, F, F, F) {
    let t41669 = t30830 * t913 * t10256;
    let t41672 = t9263 * t3358 * t2482;
    let t41674 = t31356 * t12957;
    let t41676 = t35216 * t9287;
    let t41683 = t9263 * t2792 * t3177;
    (t41669, t41672, t41674, t41676, t41683)
}
