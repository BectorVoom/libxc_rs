//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1279/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1279<F: Float>(t11046: F, t11050: F, t11053: F, t11055: F, t11057: F, t11063: F, t11065: F, t11069: F, t11073: F, t11074: F, t11079: F, t11081: F, t11088: F, t12718: F) -> F {
    let t15035 = t11046 / F::new(3.0) + F::new(0.06077777777777778) * t11050 + t11053 + F::new(0.36466666666666664) * t11055 + F::new(2.0) * t11057 + t11063 + F::new(0.004546314527777778) * t11065 + t11069 + t11073 + F::new(0.547) * t11074 + t11079 + t11081 + t11088 - t12718;
    t15035
}
