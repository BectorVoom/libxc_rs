//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1170/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1170<F: Float>(t6303: F, t822: F, t17058: F, t17060: F, t17063: F, t11057: F, t11060: F, t11063: F, t11065: F, t11069: F, t11073: F, t11074: F, t11079: F, t11081: F, t11088: F) -> (F, F, F, F, F) {
    let t21362 = t822 * t6303;
    let t21363 = F::new(8.0) / F::new(15.0) * t21362;
    let t21364 = F::new(8.0) / F::new(15.0) * t17058;
    let t21365 = F::new(8.0) / F::new(15.0) * t17060;
    let t21366 = F::new(8.0) / F::new(15.0) * t17063;
    let t21372 = t21363 + t21364 + t21365 + t21366 + F::new(2.0) / F::new(3.0) * t11057 + 2e-21 * t11060 + t11063 + F::new(0.001515438175925926) * t11065 + t11069 + t11073 + F::new(0.18233333333333332) * t11074 + t11079 + t11081 / F::new(3.0) + t11088;
    (t21363, t21364, t21365, t21366, t21372)
}
