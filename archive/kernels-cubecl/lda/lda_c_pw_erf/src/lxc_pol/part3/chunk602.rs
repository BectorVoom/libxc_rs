//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 602/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk602<F: Float>(t2017: F, t3429: F, t1318: F, t1529: F, t565: F, t1524: F, t568: F, t2070: F, t220: F) -> (F, F, F, F, F, F, F) {
    let t3430 = t2017 * t3429;
    let t3432 = F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t1318 * t3430;
    let t3433 = t565 * t1529;
    let t3434 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t3433;
    let t3435 = t1524 * t568;
    let t3436 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t3435;
    let t3437 = t2070 * t220;
    (t3430, t3432, t3433, t3434, t3435, t3436, t3437)
}
