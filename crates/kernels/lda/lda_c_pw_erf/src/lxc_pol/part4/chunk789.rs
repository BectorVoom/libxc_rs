//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 789/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk789<F: Float>(t3158: F, t3161: F, t3169: F, t3171: F, t3173: F, t3179: F, t3183: F, t3185: F, t3187: F, t3133: F, t3139: F, t3151: F, t3157: F, t3168: F, t3176: F, t3192: F) -> (F, F, F, F, F, F) {
    let t5707 = 48.0 * t3158;
    let t5708 = 80.0 * t3161;
    let t5709 = 12.0 * t3169;
    let t5710 = 24.0 * t3171;
    let t5711 = 32.0 * t3173;
    let t5712 = 40.0 * t3179;
    let t5713 = 16.0 * t3183;
    let t5714 = 4.0 * t3185;
    let t5715 = 4.0 * t3187;
    let t5716 = t3133 - t3139 + t3151 - t3157 + t5707 + t5708 - t3168 - t5709 - t5710 - t5711 + t3176 + t5712 - t5713 - t5714 - t5715 + t3192;
    (t5707, t5708, t5709, t5710, t5711, t5716)
}
