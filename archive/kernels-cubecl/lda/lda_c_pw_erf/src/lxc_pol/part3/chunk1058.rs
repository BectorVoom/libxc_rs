//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1058/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1058<F: Float>(t12387: F, t12389: F, t3965: F, t11857: F, t4488: F, t3412: F, t4483: F, t12323: F, t494: F, t6710: F, t1251: F, t4489: F) -> (F, F, F, F, F) {
    let t12392 = F::cast_from(16.0_f64) / F::cast_from(5.0_f64) * t3965 * t12387 * t12389;
    let t12395 = F::cast_from(8.0_f64) / F::cast_from(5.0_f64) * t4488 * t12387 * t11857;
    let t12398 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t4488 * t4483 * t3412;
    let t12402 = F::cast_from(16.0_f64) / F::cast_from(15.0_f64) * t3965 * t6710 * t12323 * t494;
    let t12403 = t4489 * t1251;
    (t12392, t12395, t12398, t12402, t12403)
}
