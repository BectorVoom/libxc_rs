//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 833/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk833<F: Float>(t123: F, t317: F, t4001: F, t902: F, t113: F, t1798: F, t247: F, t301: F, t1147: F, t2164: F, t2257: F, t26: F, t329: F, t413: F, t5567: F, t642: F, t794: F) -> (F, F, F, F, F, F) {
    let t11624 = t123 * t4001 * t902 * t317;
    let t11628 = t247 * t1798 * t113 * t301;
    let t11629 = 0.004067943812504169 * t11628;
    let t11632 = t123 * t1147 * t2164 * t317;
    let t11633 = 0.5945049527603057 * t11632;
    let t11639 = t26 * t2257;
    let t11640 = t329 * t11639;
    let t11674 = t5567 * t413 * t301;
    let t11676 = t642 * t794;
    (t11624, t11629, t11633, t11640, t11674, t11676)
}
