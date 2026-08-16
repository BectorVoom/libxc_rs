//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1121/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1121(t21187: f64, t3799: f64, t21183: f64, t41458: f64, t420: f64, t701: f64, t88252: f64, t2441: f64, t88239: f64, t41537: f64, t704: f64, t86571: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t88542 = t3799 * t21187;
    let t88544 = t3799 * t21183;
    let t88548 = t701 * t420 * t41458 * t88252;
    let t88552 = t701 * t420 * t2441 * t88239;
    let t88556 = t701 * t420 * t41537 * t88252;
    let t88560 = t701 * t420 * t704 * t86571;
    (t88542, t88544, t88548, t88552, t88556, t88560)
}
