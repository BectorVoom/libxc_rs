//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 836/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk836<F: Float>(t205: F, t7364: F, t208: F, t213: F, t4079: F, t4082: F, t4102: F, t4105: F, t4115: F, t4117: F, t4121: F, t4151: F, t7721: F, t7723: F, t7728: F, t7730: F, t7732: F, t7734: F, t7738: F, t7744: F) -> (F, F, F) {
    let t7974 = t7364 * t205;
    let t7975 = t7974 * t208;
    let t7978 = t4079 + t4082 - t4102 + t4105 + t4115 + t4117 - t4121 + t7721 + t7723 - t4151 + t7728 + t7730 - t7732 - t7734 - t7738 + t7975 * t213 / F::new(3.0) - t7744;
    (t7974, t7975, t7978)
}
