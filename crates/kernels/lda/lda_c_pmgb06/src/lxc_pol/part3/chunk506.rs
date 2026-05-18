//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 506/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk506<F: Float>(t12: F, t1079: F, t764: F, t1: F, t14: F, t337: F, t395: F, t2132: F, t257: F, zeta_threshold: F) -> (F, F, F) {
    let t13 = t12 <= zeta_threshold;
    let t2133 = t1079 * t764;
    let t2136 = t14 * t1;
    let t2140 = piecewise3::<f64>(t13, F::new(0.0), F::new(4.0) / F::new(9.0) * t2133 * t337 - F::new(8.0) / F::new(3.0) * t2136 * t395);
    let t2142 = (t2132 + t2140) * t257;
    (t2133, t2136, t2142)
}
