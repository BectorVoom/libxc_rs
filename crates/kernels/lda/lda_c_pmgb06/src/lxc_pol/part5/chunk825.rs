//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 825/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk825<F: Float>(t3960: F, t4549: F, t3966: F, t1122: F, t2142: F, t30: F, t3963: F, t8781: F, t8785: F, t1105: F, t2160: F, t2158: F, t8799: F, t2148: F, t3729: F, t3725: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t11117 = t4549 * t3960;
    let t11119 = t4549 * t3966;
    let t11122 = t2142 * t30 * t1122;
    let t11123 = 0.03253074390090522 * t11122;
    let t11124 = t4549 * t3963;
    let t11132 = 960.0 * t8781;
    let t11133 = 192.0 * t8785;
    let t11135 = t1105 * t2160;
    let t11136 = 36.0 * t11135;
    let t11139 = t1105 * t2158;
    let t11140 = 36.0 * t11139;
    let t11141 = 96.0 * t8799;
    let t11142 = t2148 * t3729;
    let t11147 = t2148 * t3725;
    (t11117, t11119, t11123, t11124, t11132, t11133, t11136, t11140, t11141, t11142, t11147)
}
