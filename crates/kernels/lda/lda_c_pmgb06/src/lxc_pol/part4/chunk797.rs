//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 797/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk797<F: Float>(t12: F, t1072: F, t14: F, t2133: F, t337: F, t5974: F, t6054: F, t6059: F, t257: F, t6053: F, zeta_threshold: F) -> (F,) {
    let t13 = t12 <= zeta_threshold;
    let t6065 = piecewise3(t13, 0.0, -8.0 / 27.0 * t6054 * t337 - 16.0 / 9.0 * t2133 * t1072 + 4.0 / 9.0 * t6059 * t337 + 4.0 / 3.0 * t14 * t5974);
    let t6067 = (t6053 + t6065) * t257;
    (t6067,)
}
