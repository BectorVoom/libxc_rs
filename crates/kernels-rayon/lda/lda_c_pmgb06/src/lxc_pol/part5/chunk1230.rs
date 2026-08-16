//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1230/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1230(t208: f64, t213: f64, t579: f64, t7364: f64, t588: f64, t7974: f64, t97: f64, t12753: f64, t18244: f64, t19130: f64, t20155: f64, t20159: f64, t20161: f64, t20162: f64, t20165: f64, t20168: f64, t20171: f64, t205: f64) -> f64 {
    let t21964 = t7364 * t579 * t208 * t213;
    let t21967 = t7974 * t97 * t588;
    let t21970 = -t20155 + t20159 + t19130 * t205 * t208 * t213 / 3.0_f64 + t21964 / 3.0_f64 + 0.06077777777777778_f64 * t21967 - t20161 - t20162 + t12753 + 0.09973633333333333_f64 * t18244 - t20165 - t20168 + t20171;
    t21970
}
