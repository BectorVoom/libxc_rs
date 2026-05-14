//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 830/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk830<F: Float>(t10436: F, t211: F, t197: F, t3783: F, t529: F, t4048: F, t9: F, t3892: F, t1245: F, t187: F, t22: F, t1484: F, t155: F, t219: F, t3762: F, t156: F, t4195: F, t602: F) -> (F, F, F, F, F, F, F, F, F) {
    let t10438 = 112.0 / 1215.0 * t211 * t10436;
    let t10463 = t3783 * t197;
    let t10467 = t3783 * t529;
    let t10527 = t9 * t4048;
    let t10557 = t9 * t3892;
    let t10567 = t22 / t187 / t1245;
    let t10605 = t155 * t1484;
    let t10654 = t3762 * t219;
    let t10675 = 0.4328416544945937 * t602 * t156 * t4195;
    (t10438, t10463, t10467, t10527, t10557, t10567, t10605, t10654, t10675)
}
