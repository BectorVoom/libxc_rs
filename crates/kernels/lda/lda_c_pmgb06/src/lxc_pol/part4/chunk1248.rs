//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1248/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1248<F: Float>(t1271: F, t2715: F, t955: F, t1238: F, t2696: F, t348: F, t350: F, t7015: F, t2699: F, t18568: F, t35: F, t64: F, t14816: F, t370: F, t2707: F, t410: F) -> (F, F, F, F, F, F, F) {
    let t18725 = t1271 * t2715 * t955;
    let t18728 = t1238 * t2696 * t955;
    let t18729 = 0.6495611111111111 * t18728;
    let t18731 = t348 * t7015 * t350;
    let t18732 = 0.9743416666666667 * t18731;
    let t18734 = t1238 * t2699 * t955;
    let t18735 = 0.3247805555555556 * t18734;
    let t18737 = t35 * t64 * t18568;
    let t18741 = t35 * t370 * t14816;
    let t18744 = t410 * t2707;
    (t18725, t18729, t18732, t18735, t18737, t18741, t18744)
}
