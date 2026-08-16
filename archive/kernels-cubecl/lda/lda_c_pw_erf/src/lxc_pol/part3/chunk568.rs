//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 568/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk568<F: Float>(t3010: F, t2761: F, t2944: F, t2950: F, t2952: F, t2981: F, t2989: F, t2991: F, t2995: F, t3000: F, t3003: F, t3005: F, t3009: F) -> F {
    let t3011 = F::cast_from(12.0_f64) * t3010;
    let t3012 = -t2761 - t2944 + t2950 + t2952 + t2981 - t2989 - t2991 + t2995 - t3000 - t3003 + t3005 - t3009 + t3011;
    t3012
}
