//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 905/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk905<F: Float>(t3257: F, t8998: F, t1124: F, t119: F, t411: F, t1657: F, t3267: F, t1691: F, t435: F, t97: F, t3338: F, t440: F) -> (F, F, F, F, F, F) {
    let t8999 = t3257 * t8998;
    let t9002 = t119 * t1124 * t411;
    let t9003 = t1657 * t9002;
    let t9017 = t3267 * t8998;
    let t9019 = t1691 * t9002;
    let t9037 = F::new(1.0) / t435 / t97;
    let t9059 = t440 * t3338;
    (t8999, t9003, t9017, t9019, t9037, t9059)
}
