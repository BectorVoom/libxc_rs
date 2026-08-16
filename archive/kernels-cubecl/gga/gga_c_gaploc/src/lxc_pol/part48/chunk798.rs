//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 798/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk798<F: Float>(t27003: F, t587: F, t9438: F, t12965: F, t1407: F, t41634: F, t912: F, t2487: F, t27007: F, t26328: F, t6914: F, t1365: F, t31558: F, t6525: F) -> (F, F, F, F, F, F) {
    let t42420 = t587 * t9438 * t27003;
    let t42422 = t1407 * t12965;
    let t42425 = t587 * t912 * t41634;
    let t42428 = t2487 * t9438 * t27007;
    let t42431 = t6914 * t9438 * t26328;
    let t42529 = t6525 * t1365 * t31558;
    (t42420, t42422, t42425, t42428, t42431, t42529)
}
