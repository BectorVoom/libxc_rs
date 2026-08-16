//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 484/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk484(t2725: f64, t2726: f64, t2434: f64, t2437: f64, t2444: f64, t2449: f64, t2453: f64) -> (f64, f64) {
    let t2727 = t2725 * t2726;
    let t2730 = 0.11113000182098765433e-1_f64 * t2434;
    let t2735 = -t2730 + 0.11113000182098765433e-1_f64 * t2437 + 0.22226000364197530865e-1_f64 * t2444 - 0.33339000546296296298e-1_f64 * t2449 + 0.16669500273148148149e-1_f64 * t2453;
    (t2727, t2735)
}
