//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 468/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk468<F: Float>(t668: F, t858: F, t739: F, t92: F, t34: F, t659: F, t743: F, t93: F, t661: F, t108: F, t348: F, t352: F, t462: F) -> (F, F, F, F) {
    let t2266 = t858 * t668;
    let t2268 = t92 * t739;
    let t2271 = t659 * t34;
    let t2274 = t93 * t743;
    let t2277 = t661 * t34;
    let t2281 = (F::cast_from(20.0_f64) / F::cast_from(9.0_f64) * t2268 * t348 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t2271 * t462 + F::cast_from(20.0_f64) / F::cast_from(9.0_f64) * t2274 * t352 - F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t2277 * t462) * t108;
    (t2266, t2268, t2274, t2281)
}
