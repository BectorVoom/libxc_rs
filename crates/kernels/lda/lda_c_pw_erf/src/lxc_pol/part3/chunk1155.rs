//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1155/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1155<F: Float>(t1184: F, t2177: F, t519: F, t521: F, t1321: F, t2065: F, t3974: F, t3975: F, t3424: F, t5151: F, t3420: F, t13384: F, t3429: F) -> (F, F, F, F, F) {
    let t13523 = t519 * t1184 * t521 * t2177;
    let t13524 = F::new(128.0) / F::new(135.0) * t13523;
    let t13528 = F::new(16.0) / F::new(15.0) * t3974 * t3975 * t2065 * t1321;
    let t13531 = F::new(8.0) / F::new(15.0) * t3974 * t5151 * t3424;
    let t13534 = F::new(8.0) / F::new(15.0) * t3974 * t5151 * t3420;
    let t13537 = F::new(8.0) / F::new(9.0) * t3974 * t13384 * t3429;
    (t13524, t13528, t13531, t13534, t13537)
}
