//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1995/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1995(t10566: f64, t10568: f64, t14333: f64, t14335: f64, t14337: f64, t14340: f64, t14343: f64, t14345: f64, t14352: f64, t14364: f64, t14372: f64, t14373: f64, t14374: f64, t14379: f64, t14380: f64, t9394: f64) -> f64 {
    let t14610 = t14333 - t14335 - t14337 + t14340 + t14343 + t14345 + t14352 + t9394 + t14364 + t14372 + t14373 + t14374 + t10566 - t10568 + t14379 - t14380;
    t14610
}
