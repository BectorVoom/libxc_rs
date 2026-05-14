//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 823/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk823<F: Float>(t3004: F, t2995: F, t3000: F, t3009: F, t3016: F, t3018: F, t3118: F, t3121: F, t3125: F, t3133: F, t3139: F, t3151: F, t3155: F, t5698: F, t5703: F, t5704: F) -> (F,) {
    let t6063 = 0.0002441540671567088 * t3004;
    let t6064 = -t5698 + t2995 - t3000 + t6063 - t3009 - t5703 - t5704 + t3016 + t3018 + t3155 + t3118 - t3121 + t3125 + t3133 - t3139 + t3151;
    (t6064,)
}
