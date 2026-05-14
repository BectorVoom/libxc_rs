//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 635/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk635<F: Float>(t50: F, t1789: F, t352: F, t52: F, t5997: F, t6002: F, t6005: F, t943: F, t59: F, t5996: F, zeta_threshold: F) -> (F,) {
    let t51 = t50 <= zeta_threshold;
    let t6009 = piecewise3(t51, 0.0, -8.0 / 27.0 * t5997 * t352 - 16.0 / 9.0 * t1789 * t943 + 4.0 / 9.0 * t6002 * t352 + 4.0 / 3.0 * t52 * t6005);
    let t6011 = (t5996 + t6009) * t59;
    (t6011,)
}
