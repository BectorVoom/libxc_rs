//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 783/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk783<F: Float>(t3966: F, t3969: F, t110: F, t1121: F, t3760: F, t410: F, t980: F, t3705: F, t1180: F, t698: F, t1025: F, t3675: F, t3868: F, t633: F, t964: F) -> (F, F, F, F, F, F, F, F, F) {
    let t8576 = t3969 * t3966;
    let t8580 = 0.02168716260060348 * t1121 * t110 * t3760;
    let t8583 = 0.08674865040241392 * t1121 * t410 * t980;
    let t8586 = 0.13012297560362088 * t1121 * t110 * t3705;
    let t8589 = 0.06747117253521083 * t1121 * t1180 * t698;
    let t8594 = 578.9512619529313 * t3675 * t3868 * t1025;
    let t8595 = t1025 * t1025;
    let t8598 = 24.0 * t3675 * t8595 * t633;
    let t8599 = t964 * t964;
    (t8576, t8580, t8583, t8586, t8589, t8594, t8595, t8598, t8599)
}
