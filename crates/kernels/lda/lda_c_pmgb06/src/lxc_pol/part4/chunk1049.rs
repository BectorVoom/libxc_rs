//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1049/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1049<F: Float>(t4524: F, t643: F, t638: F, t3957: F, t4549: F, t3960: F, t3966: F, t1122: F, t2142: F, t30: F, t3963: F, t1105: F, t2160: F) -> (F, F, F, F, F, F, F, F) {
    let t11110 = t643 * t4524;
    let t11112 = t638 * t4524;
    let t11115 = t4549 * t3957;
    let t11117 = t4549 * t3960;
    let t11119 = t4549 * t3966;
    let t11122 = t2142 * t30 * t1122;
    let t11124 = t4549 * t3963;
    let t11135 = t1105 * t2160;
    (t11110, t11112, t11115, t11117, t11119, t11122, t11124, t11135)
}
