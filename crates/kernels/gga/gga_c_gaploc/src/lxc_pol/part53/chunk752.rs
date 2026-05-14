//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 752/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk752<F: Float>(t12990: F, t30733: F, t27003: F, t587: F, t9438: F, t2487: F, t27007: F, t26328: F, t6914: F, t12891: F, t1580: F, t1445: F, t3085: F, t597: F, t7995: F, t11392: F, t3159: F) -> (F, F, F, F, F, F, F) {
    let t42412 = t12990 * t30733;
    let t42413 = 0.59584149919750711116e-1 * t42412;
    let t42420 = t587 * t9438 * t27003;
    let t42421 = 0.31952438294933958064e-1 * t42420;
    let t42428 = t2487 * t9438 * t27007;
    let t42429 = 0.7988109573733489516e-1 * t42428;
    let t42431 = t6914 * t9438 * t26328;
    let t42432 = 0.47928657442400937096e-1 * t42431;
    let t42438 = 0.43710935587469654631e2 * t1580 * t12891;
    let t42442 = 0.43710935587469654631e2 * t597 * t1445 * t7995 * t3085;
    let t42444 = 0.25025342966295298669e1 * t3159 * t11392;
    (t42413, t42421, t42429, t42432, t42438, t42442, t42444)
}
