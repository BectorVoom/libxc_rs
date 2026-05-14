//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 589/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk589<F: Float>(t12: F, t14: F, t158: F, t1219: F, t337: F, t1083: F, t2912: F, t2938: F, t336: F, t3546: F, zeta_threshold: F) -> (F, F) {
    let t13 = t12 <= zeta_threshold;
    let t3548 = 1.0 / t14 / t158;
    let t3551 = t1219 * t337;
    let t3557 = piecewise3(t13, 0.0, 8.0 / 27.0 * t3548 * t2912 - 2.0 / 3.0 * t3551 * t1083 + 2.0 / 3.0 * t336 * t2938);
    let t3559 = t3546 / 2.0 + t3557 / 2.0;
    (t3548, t3559)
}
