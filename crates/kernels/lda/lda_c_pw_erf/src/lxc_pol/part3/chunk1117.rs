//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1117/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1117<F: Float>(t13066: F, t4581: F, t4753: F, t3416: F, t1318: F, t3854: F, t5225: F, t13042: F, t13044: F, t13046: F, t13049: F, t13052: F, t13055: F, t13057: F, t13059: F, t13064: F) -> (F, F, F, F, F) {
    let t13067 = F::new(8.0) / F::new(27.0) * t13066;
    let t13068 = t4753 * t4581;
    let t13069 = F::new(32.0) / F::new(45.0) * t13068;
    let t13070 = t3416 * t4581;
    let t13071 = F::new(32.0) / F::new(45.0) * t13070;
    let t13073 = t1318 * t3854 * t5225;
    let t13074 = F::new(16.0) / F::new(45.0) * t13073;
    let t13075 = -t13042 - t13044 - t13046 + t13049 + t13052 - t13055 - t13057 + t13059 - t13064 - t13067 + t13069 + t13071 + t13074;
    (t13067, t13069, t13071, t13074, t13075)
}
