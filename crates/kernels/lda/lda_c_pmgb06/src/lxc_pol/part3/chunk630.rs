//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 630/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk630<F: Float>(t5: F, t12: F, t10: F, t332: F, t1074: F, t3010: F, t3115: F, t330: F, t594: F, t15: F, t337: F, t1083: F, t2912: F, t2938: F, t336: F, t598: F, t44: F, t1727: F, t607: F, zeta_threshold: F) -> (F, F) {
    let t6 = t5 <= zeta_threshold;
    let t13 = t12 <= zeta_threshold;
    let t4125 = t10 * t332;
    let t4131 = piecewise3(t6, 0.0, 80.0 / 27.0 * t330 * t3010 + 40.0 / 3.0 * t4125 * t1074 + 8.0 / 3.0 * t594 * t3115);
    let t4134 = t15 * t337;
    let t4140 = piecewise3(t13, 0.0, 80.0 / 27.0 * t336 * t2912 + 40.0 / 3.0 * t4134 * t1083 + 8.0 / 3.0 * t598 * t2938);
    let t4143 = (t4131 / 2.0 + t4140 / 2.0) * t44;
    let t4146 = t1727 * t607;
    (t4143, t4146)
}
