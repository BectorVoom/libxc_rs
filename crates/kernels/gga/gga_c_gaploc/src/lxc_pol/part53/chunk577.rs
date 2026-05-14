//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 577/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk577<F: Float>(t10019: F, t2617: F, t3005: F, t7810: F, t10628: F, t4820: F, t7513: F, t1029: F, t7803: F, t3073: F, t977: F, t1645: F, t2792: F, t2963: F, t3689: F, t555: F) -> (F, F, F, F, F, F, F, F) {
    let t11108 = 0.15976219147466979032e-1 * t10019;
    let t11109 = t3005 * t2617;
    let t11110 = t7810 * t11109;
    let t11111 = 0.19171462976960374838e0 * t11110;
    let t11116 = t4820 * t10628;
    let t11118 = 0.79445533226334281487e-1 * t7513 * t11116;
    let t11119 = t1029 * t2617;
    let t11120 = t7803 * t11119;
    let t11121 = 0.19171462976960374838e0 * t11120;
    let t11135 = t3073 * t977;
    let t11392 = t1645 * t2792;
    let t11807 = t1645 * t2963;
    let t11977 = t555 * t3689;
    (t11108, t11111, t11118, t11121, t11135, t11392, t11807, t11977)
}
