//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1230/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1230<F: Float>(t208: F, t213: F, t579: F, t7364: F, t588: F, t7974: F, t97: F, t12753: F, t18244: F, t19130: F, t20155: F, t20159: F, t20161: F, t20162: F, t20165: F, t20168: F, t20171: F, t205: F) -> F {
    let t21964 = t7364 * t579 * t208 * t213;
    let t21967 = t7974 * t97 * t588;
    let t21970 = -t20155 + t20159 + t19130 * t205 * t208 * t213 / F::new(3.0) + t21964 / F::new(3.0) + F::new(0.06077777777777778) * t21967 - t20161 - t20162 + t12753 + F::new(0.09973633333333333) * t18244 - t20165 - t20168 + t20171;
    t21970
}
