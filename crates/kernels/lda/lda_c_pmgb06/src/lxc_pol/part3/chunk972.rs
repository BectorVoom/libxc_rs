//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 972/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk972<F: Float>(t11296: F, t11297: F, t1227: F, t2209: F, t2247: F, t2248: F, t342: F, t3559: F, t4394: F, t769: F, t8263: F, t8287: F, t8295: F, t8431: F, t8433: F, t8435: F, t8439: F, t8441: F) -> F {
    let t11511 = F::new(15.518295) * t2247 * t2248 * t4394 * t342 + F::new(15.518295) * t2247 * t2248 * t2209 * t1227 + F::new(5.172765) * t2247 * t2248 * t769 * t3559 + F::new(0.5747516666666667) * t8431 + F::new(6.89702) * t8433 + F::new(6.89702) * t8435 + F::new(5.364348888888889) * t8439 - F::new(2.2990066666666666) * t8441 - t11296 + t8263 + t11297 + t8287 - t8295;
    t11511
}
