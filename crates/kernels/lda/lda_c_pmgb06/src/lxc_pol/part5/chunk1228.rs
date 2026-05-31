//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1228/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1228<F: Float>(t12657: F, t12662: F, t20109: F, t20111: F, t20112: F, t20113: F, t20115: F, t20116: F, t20121: F, t20122: F, t20123: F, t20127: F) -> F {
    let t21955 = t20109 + t20111 + t20112 + t20113 - t20115 + t20116 - F::cast_from(8.0_f64) / F::cast_from(135.0_f64) * t12657 + t12662 + t20121 + t20122 + t20123 - t20127;
    t21955
}
