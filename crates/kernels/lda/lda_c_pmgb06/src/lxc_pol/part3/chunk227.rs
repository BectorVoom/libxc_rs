//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 227/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk227<F: Float>(t5: F, t12: F, t286: F, t643: F, t332: F, t9: F, t14: F, t337: F, t257: F, zeta_threshold: F) -> (F, F) {
    let t6 = t5 <= zeta_threshold;
    let t13 = t12 <= zeta_threshold;
    let t645 = 4.0 * t643 * t286;
    let t648 = piecewise3(t6, 0.0, 4.0 / 3.0 * t9 * t332);
    let t651 = piecewise3(t13, 0.0, 4.0 / 3.0 * t14 * t337);
    let t653 = (t648 + t651) * t257;
    (t645, t653)
}
