//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3080/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3080<F: Float>(t44348: F, t52011: F, t77513: F, t81171: F, t81175: F, t81179: F, t81184: F, t81188: F, t81192: F, t81196: F, t81200: F, t81204: F, t81209: F, t81214: F) -> (F, F) {
    let t81416 = t52011 * t44348 * t77513;
    let t81418 = -F::cast_from(0.35876000000000000001e1_f64) * t81171 - F::cast_from(0.71752000000000000002e1_f64) * t81175 - F::cast_from(0.59793333333333333333e0_f64) * t81179 - F::cast_from(0.19931111111111111111e0_f64) * t81184 - F::cast_from(0.59793333333333333333e0_f64) * t81188 + F::cast_from(0.53814e1_f64) * t81192 + F::cast_from(0.71752e1_f64) * t81196 + F::cast_from(0.17938e1_f64) * t81200 + F::cast_from(0.17938e1_f64) * t81204 + F::cast_from(0.59793333333333333334e0_f64) * t81209 - F::cast_from(0.88582716049382716048e0_f64) * t81214 + F::cast_from(0.10954222222222222222e0_f64) * t81416;
    (t81416, t81418)
}
