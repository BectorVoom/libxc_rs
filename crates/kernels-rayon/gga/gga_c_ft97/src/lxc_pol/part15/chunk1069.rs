//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1069/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1069(t17239: f64, t4778: f64, t91: f64, t1969: f64, t446: f64, t86610: f64, t86902: f64, t40294: f64, t7761: f64, t85469: f64, t89: f64, t86665: f64, t9073: f64) -> (f64, f64, f64, f64, f64) {
    let t87056 = t91 * t17239 * t4778;
    let t87060 = t446 * t1969 * t86610;
    let t87063 = t446 * t1969 * t86902;
    let t87067 = t89 * t7761 * t40294 * t85469;
    let t87071 = t446 * t9073 * t86665;
    (t87056, t87060, t87063, t87067, t87071)
}
