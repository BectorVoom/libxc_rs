//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1484/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1484(t41341: f64, t41344: f64, t41347: f64, t41350: f64, t41353: f64, t41356: f64, t41359: f64, t41361: f64, t41363: f64, t41365: f64, t41367: f64, t41369: f64) -> f64 {
    let t42096 = -0.43901234567901234568e-1_f64 * t41341 - 0.14816666666666666667e-1_f64 * t41344 - 0.35560000000000000001e0_f64 * t41347 + 0.19755555555555555556e0_f64 * t41350 - 0.98777777777777777779e-1_f64 * t41353 + 0.11853333333333333334e0_f64 * t41356 - 0.39511111111111111112e-1_f64 * t41359 + 0.61461728395061728396e-1_f64 * t41361 + 0.79022222222222222224e-1_f64 * t41363 - 0.11853333333333333334e0_f64 * t41365 + 0.39511111111111111112e-1_f64 * t41367 - 0.79022222222222222224e-1_f64 * t41369;
    t42096
}
