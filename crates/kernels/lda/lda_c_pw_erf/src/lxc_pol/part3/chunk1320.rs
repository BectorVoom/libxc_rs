//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1320/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1320<F: Float>(t5833: F, t668: F, t14307: F, t14311: F, t14314: F, t14317: F, t14319: F, t14321: F, t14323: F, t14327: F, t14329: F, t14331: F, t14333: F, t14338: F) -> F {
    let t15204 = t5833 * t668;
    let t15206 = -F::cast_from(2.0_f64) / F::cast_from(15.0_f64) * t15204 + t14307 - t14311 + t14314 - t14317 + t14319 - t14321 - t14323 - t14327 - t14329 - t14331 + t14333 - t14338;
    t15206
}
