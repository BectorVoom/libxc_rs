//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 982/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk982<F: Float>(t7475: F, t2782: F, t3661: F, t10: F, t3658: F, t1096: F, t7483: F, t7493: F, t7479: F, t7481: F, t7489: F, t7491: F, t7495: F, t7527: F, t7528: F, t7538: F, t7541: F, t7544: F, t9245: F) -> (F, F, F, F, F, F, F) {
    let t9613 = 4.0 * t7475;
    let t9614 = t3661 * t2782;
    let t9616 = t3658 * t10;
    let t9618 = 0.36622894612013090108e-3 * t9616 * t1096;
    let t9619 = 80.0 * t7483;
    let t9620 = 48.0 * t7493;
    let t9625 = -t9245 + t9613 - t7479 + t7481 + 0.24415263074675393405e-3 * t9614 - t9618 + t9619 - t7489 - t7491 + t9620 - 0.11696447245269292414e1 * t7495 - t7527 - 0.34631718211362927518e2 * t7528 - t7538 - 0.5848223622634646207e0 * t7541 + 2.0 * t7544;
    (t9613, t9614, t9616, t9618, t9619, t9620, t9625)
}
