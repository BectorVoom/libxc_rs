//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 860/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk860<F: Float>(t108: F, t2268: F, t2274: F, t2329: F, t2337: F, t406: F, t408: F, t659: F, t661: F, t7354: F, t7360: F, t7365: F, t7370: F) -> F {
    let t8025 = (F::cast_from(40.0_f64) / F::cast_from(27.0_f64) * t406 * t7354 + F::cast_from(20.0_f64) / F::cast_from(3.0_f64) * t2268 * t2329 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t659 * t7360 + F::cast_from(40.0_f64) / F::cast_from(27.0_f64) * t408 * t7365 + F::cast_from(20.0_f64) / F::cast_from(3.0_f64) * t2274 * t2337 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t661 * t7370) * t108;
    t8025
}
