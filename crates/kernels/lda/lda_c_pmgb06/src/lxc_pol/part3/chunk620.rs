//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 620/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk620<F: Float>(t5: F, t12: F, t1074: F, t3010: F, t3115: F, t330: F, t3537: F, t3540: F, t14: F, t158: F, t1219: F, t337: F, t1083: F, t2912: F, t2938: F, t336: F, zeta_threshold: F) -> (F, F, F) {
    let t6 = t5 <= zeta_threshold;
    let t13 = t12 <= zeta_threshold;
    let t3546 = piecewise3::<f64>(t6, F::new(0.0), F::new(8.0) / F::new(27.0) * t3537 * t3010 - F::new(2.0) / F::new(3.0) * t3540 * t1074 + F::new(2.0) / F::new(3.0) * t330 * t3115);
    let t3548 = F::new(1.0) / t14 / t158;
    let t3551 = t1219 * t337;
    let t3557 = piecewise3::<f64>(t13, F::new(0.0), F::new(8.0) / F::new(27.0) * t3548 * t2912 - F::new(2.0) / F::new(3.0) * t3551 * t1083 + F::new(2.0) / F::new(3.0) * t336 * t2938);
    (t3546, t3548, t3557)
}
