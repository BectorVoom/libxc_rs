//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 550/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk550<F: Float>(t3259: F, t747: F, t3263: F, t841: F, t2728: F, t977: F, t3322: F, t2617: F, t948: F, t7803: F, t3251: F, t590: F) -> (F, F, F, F, F, F) {
    let t9767 = t3259 * t747;
    let t9777 = t3263 * t841;
    let t9780 = t977 * t2728;
    let t9784 = t3322 * t841;
    let t9787 = t948 * t2617;
    let t9788 = t7803 * t9787;
    let t9789 = F::cast_from(0.38342925953920749676e0_f64) * t9788;
    let t9790 = t3251 * t590;
    (t9767, t9777, t9780, t9784, t9789, t9790)
}
