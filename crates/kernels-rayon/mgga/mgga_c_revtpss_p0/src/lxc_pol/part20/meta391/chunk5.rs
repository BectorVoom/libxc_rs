//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1443/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1443(t41341: f64, t41344: f64, t41347: f64, t41350: f64, t41353: f64, t41356: f64, t41359: f64, t41361: f64, t41363: f64, t41365: f64, t41367: f64, t41369: f64) -> f64 {
    let t41371 = -80.0_f64 / 81.0_f64 * t41341 - t41344 / 3.0_f64 - 8.0_f64 * t41347 + 40.0_f64 / 9.0_f64 * t41350 - 20.0_f64 / 9.0_f64 * t41353 + 8.0_f64 / 3.0_f64 * t41356 - 8.0_f64 / 9.0_f64 * t41359 + 112.0_f64 / 81.0_f64 * t41361 + 16.0_f64 / 9.0_f64 * t41363 - 8.0_f64 / 3.0_f64 * t41365 + 8.0_f64 / 9.0_f64 * t41367 - 16.0_f64 / 9.0_f64 * t41369;
    t41371
}
