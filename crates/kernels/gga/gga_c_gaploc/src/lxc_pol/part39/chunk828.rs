//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 828/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk828<F: Float>(t41634: F, t587: F, t912: F, t2487: F, t27007: F, t9438: F, t26328: F, t6914: F, t41838: F, t6508: F, t4820: F, t6824: F, t12891: F, t1580: F, t1445: F, t3085: F, t597: F, t7995: F) -> (F, F, F, F, F, F, F) {
    let t42425 = t587 * t912 * t41634;
    let t42428 = t2487 * t9438 * t27007;
    let t42429 = 0.7988109573733489516e-1 * t42428;
    let t42431 = t6914 * t9438 * t26328;
    let t42432 = 0.47928657442400937096e-1 * t42431;
    let t42433 = t6508 * t41838;
    let t42435 = t6824 * t4820 * t42433;
    let t42438 = 0.43710935587469654631e2 * t1580 * t12891;
    let t42442 = 0.43710935587469654631e2 * t597 * t1445 * t7995 * t3085;
    (t42425, t42429, t42432, t42433, t42435, t42438, t42442)
}
