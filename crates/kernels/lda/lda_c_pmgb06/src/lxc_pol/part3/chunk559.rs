//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 559/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk559<F: Float>(t5: F, t12: F, t132: F, t3122: F, t1542: F, t432: F, t1074: F, t332: F, t3115: F, t44: F, t131: F, t155: F, t1512: F, t460: F, t1083: F, t337: F, t2938: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F, F) {
    let t6 = t5 <= zeta_threshold;
    let t13 = t12 <= zeta_threshold;
    let t3124 = t132 * t3122 / 30.0;
    let t3126 = t432 * t1542 / 10.0;
    let t3127 = t332 * t1074;
    let t3132 = piecewise3(t6, 0.0, 2.0 * t5 * t3115 + 6.0 * t3127);
    let t3133 = t3132 * t44;
    let t3134 = t3133 * t131;
    let t3136 = t3134 * t155 / 30.0;
    let t3138 = t1512 * t460 / 10.0;
    let t3139 = t337 * t1083;
    let t3144 = piecewise3(t13, 0.0, 2.0 * t12 * t2938 + 6.0 * t3139);
    (t3124, t3126, t3127, t3133, t3134, t3136, t3138, t3139, t3144)
}
