//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 785/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk785<F: Float>(t3957: F, t3969: F, t110: F, t1121: F, t3711: F, t3960: F, t410: F, t959: F, t968: F, t3742: F, t3966: F, t3760: F, t980: F, t3705: F, t1180: F, t698: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t8555 = t3969 * t3957;
    let t8559 = 3.8527786510141255 * t1121 * t110 * t3711;
    let t8560 = t3969 * t3960;
    let t8564 = 0.04337432520120696 * t1121 * t410 * t959;
    let t8567 = 1.2842595503380418 * t1121 * t410 * t968;
    let t8570 = 38.025319932552506 * t1121 * t110 * t3742;
    let t8576 = t3969 * t3966;
    let t8580 = 0.02168716260060348 * t1121 * t110 * t3760;
    let t8583 = 0.08674865040241392 * t1121 * t410 * t980;
    let t8586 = 0.13012297560362088 * t1121 * t110 * t3705;
    let t8589 = 0.06747117253521083 * t1121 * t1180 * t698;
    (t8555, t8559, t8560, t8564, t8567, t8570, t8576, t8580, t8583, t8586, t8589)
}
