//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 690/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk690<F: Float>(t5: F, t153: F, t4680: F, t137: F, t132: F, t1: F, t10: F, t1069: F, t1074: F, t1941: F, t1944: F, t247: F, t395: F, t4367: F, t594: F, t761: F, t15: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t6 = t5 <= zeta_threshold;
    let t4681 = t4680 * t153;
    let t4682 = t137 * t4681;
    let t4684 = t132 * t4682 / 30.0;
    let t4687 = t10 * t1;
    let t4697 = piecewise3(t6, 0.0, 80.0 / 27.0 * t761 * t1069 + 160.0 / 9.0 * t4687 * t4367 + 40.0 / 9.0 * t1941 * t1074 + 16.0 / 3.0 * t594 * t395 - 16.0 * t1944 * t247);
    let t4700 = t15 * t1;
    (t4681, t4682, t4684, t4697, t4700)
}
