//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 640/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk640<F: Float>(t2030: F, t3802: F, t519: F, t2151: F, t581: F, t2176: F, t529: F, t1484: F, t473: F, t219: F, t1450: F, t2171: F) -> (F, F, F, F, F, F) {
    let t4834 = t3802 * t2030;
    let t4836 = F::cast_from(16.0_f64) / F::cast_from(135.0_f64) * t519 * t4834;
    let t4841 = t2151 * t581;
    let t4848 = t2176 * t529;
    let t4867 = t473 * t1484;
    let t4868 = t4867 * t219;
    let t4879 = F::cast_from(16.0_f64) / F::cast_from(135.0_f64) * t2171 * t1450;
    (t4834, t4836, t4841, t4848, t4868, t4879)
}
