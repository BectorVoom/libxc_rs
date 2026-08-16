//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 551/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk551(t2258: f64, t2281: f64, t2172: f64, t2175: f64, t2187: f64) -> (f64, f64, f64) {
    let t2282 = t2258 * t2281;
    let t2285 = 0.12361111111111111111e-1_f64 * t2172;
    let t2288 = t2285 - 0.18541666666666666667e-1_f64 * t2175 + 0.278125e-1_f64 * t2187;
    (t2282, t2285, t2288)
}
