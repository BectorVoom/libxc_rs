//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 836/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk836(t205: f64, t7364: f64, t208: f64, t213: f64, t4079: f64, t4082: f64, t4102: f64, t4105: f64, t4115: f64, t4117: f64, t4121: f64, t4151: f64, t7721: f64, t7723: f64, t7728: f64, t7730: f64, t7732: f64, t7734: f64, t7738: f64, t7744: f64) -> (f64, f64, f64) {
    let t7974 = t7364 * t205;
    let t7975 = t7974 * t208;
    let t7978 = t4079 + t4082 - t4102 + t4105 + t4115 + t4117 - t4121 + t7721 + t7723 - t4151 + t7728 + t7730 - t7732 - t7734 - t7738 + t7975 * t213 / 3.0_f64 - t7744;
    (t7974, t7975, t7978)
}
