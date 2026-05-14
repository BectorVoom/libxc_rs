//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 535/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk535<F: Float>(t50: F, t2966: F, t2967: F, t2970: F, t2973: F, t52: F, t954: F, t2965: F, t59: F, zeta_threshold: F) -> (F,) {
    let t51 = t50 <= zeta_threshold;
    let t2977 = piecewise3(t51, 0.0, -8.0 / 27.0 * t2966 * t2967 + 4.0 / 3.0 * t2970 * t954 + 4.0 / 3.0 * t52 * t2973);
    let t2979 = (t2965 + t2977) * t59;
    (t2979,)
}
