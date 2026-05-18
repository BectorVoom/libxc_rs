//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1100/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1100<F: Float>(t14582: F, t19739: F, t19750: F, t19753: F, t20305: F, t20318: F, t20319: F, t20324: F, t20328: F, t20329: F, t20330: F, t8936: F, t9096: F) -> F {
    let t20513 = t20305 - t8936 + t20318 - t20319 - t20324 - t20328 + t20329 - F::new(5.172765) * t19739 + F::new(20.69106) * t19750 - F::new(10.34553) * t19753 + t20330 + F::new(1.7881162962962962) * t9096 + F::new(5.364348888888889) * t14582;
    t20513
}
