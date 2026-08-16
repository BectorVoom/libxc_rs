//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3080/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3080(t44348: f64, t52011: f64, t77513: f64, t81171: f64, t81175: f64, t81179: f64, t81184: f64, t81188: f64, t81192: f64, t81196: f64, t81200: f64, t81204: f64, t81209: f64, t81214: f64) -> (f64, f64) {
    let t81416 = t52011 * t44348 * t77513;
    let t81418 = -0.35876000000000000001e1_f64 * t81171 - 0.71752000000000000002e1_f64 * t81175 - 0.59793333333333333333e0_f64 * t81179 - 0.19931111111111111111e0_f64 * t81184 - 0.59793333333333333333e0_f64 * t81188 + 0.53814e1_f64 * t81192 + 0.71752e1_f64 * t81196 + 0.17938e1_f64 * t81200 + 0.17938e1_f64 * t81204 + 0.59793333333333333334e0_f64 * t81209 - 0.88582716049382716048e0_f64 * t81214 + 0.10954222222222222222e0_f64 * t81416;
    (t81416, t81418)
}
