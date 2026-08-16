//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 613/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk613<F: Float>(t1138: F, t1145: F, t1151: F, t1753: F, t1765: F, t1772: F, t1775: F, t2172: F, t2176: F, t2267: F, t2269: F, t2747: F) -> F {
    let t2752 = -F::cast_from(0.02394846802050922_f64) * t2267 - F::cast_from(0.0005811348303577384_f64) * t2176 + F::cast_from(0.039914113367515366_f64) * t2269 - F::cast_from(0.10809180959278285_f64) * t2172 + t1138 - t1145 + t1151 + t1753 - t1765 - t1772 - t1775;
    let t2753 = t2747 + t2752;
    t2753
}
