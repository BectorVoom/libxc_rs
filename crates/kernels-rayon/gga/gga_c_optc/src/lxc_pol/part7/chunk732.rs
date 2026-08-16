//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 732/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk732(t2160: f64, t7018: f64, t155: f64, t2078: f64, t693: f64, t697: f64, t127: f64, t7003: f64, t675: f64, t146: f64, t2002: f64, t671: f64) -> (f64, f64, f64, f64, f64) {
    let t7019 = t7018 * t2160;
    let t7022 = t155 * t693 * t2078;
    let t7023 = t7022 * t697;
    let t7025 = t7003 * t127;
    let t7026 = t675 * t7025;
    let t7030 = t146 * t671 * t2002;
    (t7019, t7022, t7023, t7026, t7030)
}
