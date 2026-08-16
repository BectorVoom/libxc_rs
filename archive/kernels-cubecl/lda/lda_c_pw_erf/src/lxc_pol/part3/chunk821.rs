//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 821/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk821<F: Float>(t2995: F, t3000: F, t3009: F, t3016: F, t3118: F, t3121: F, t3125: F, t3155: F, t5694: F, t5696: F, t5698: F, t5699: F, t5700: F, t5703: F, t5704: F, t5705: F) -> F {
    let t5706 = -t5694 - t5696 + t5698 + t2995 - t3000 - t5699 + t5700 - t3009 - t5703 + t5704 + t3016 + t5705 + t3155 + t3118 - t3121 + t3125;
    t5706
}
