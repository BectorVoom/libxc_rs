//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1262/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1262<F: Float>(t11020: F, t11022: F, t11025: F, t11027: F, t11029: F, t12066: F, t12070: F, t12075: F, t12078: F, t12082: F, t12084: F, t12085: F, t12086: F) -> F {
    let t14991 = t12066 + t12070 - t12075 - t12078 + t12082 + F::new(0.299209) * t11020 - F::new(0.19947266666666666) * t11022 - t11025 + t11027 + t11029 + t12084 + t12085 - t12086;
    t14991
}
