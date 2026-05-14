//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1016/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1016<F: Float>(t6037: F, t980: F, t11135: F, t11139: F, t11142: F, t8747: F, t8749: F, t8755: F, t8759: F, t8760: F, t8762: F, t8769: F, t8774: F, t8779: F, t8783: F, t8787: F, t8794: F, t8798: F, t8799: F) -> (F,) {
    let t15015 = t6037 * t980;
    let t15019 = -8.0 * t8747 - 8.0 * t8749 - t8755 - t8759 + 7.017868347161575 * t8760 - 103.89515463408878 * t8762 + t8769 - t8774 + t8779 - 480.0 * t8783 - 0.0011393789434848518 * t8787 - t8794 - 24.0 * t11135 - 48.0 * t11139 + 1.1696447245269292 * t15015 - t8798 + 64.0 * t8799 + 7.017868347161575 * t11142;
    (t15019,)
}
