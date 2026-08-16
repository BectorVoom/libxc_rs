//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 508/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk508(t192: f64, t5053: f64, t743: f64, t2481: f64, t3908: f64, t3925: f64, t462: f64, t5099: f64, t5102: f64, t5106: f64, t5110: f64, t5114: f64, t92: f64) -> (f64, f64) {
    let t5118 = t192 * t743 * t5053;
    let t5120 = t2481 + 2.0_f64 / 9.0_f64 * t3908 + 2.0_f64 / 3.0_f64 * t3925 - 2.0_f64 / 9.0_f64 * t462 * t5099 + 2.0_f64 / 3.0_f64 * t462 * t5102 + 2.0_f64 / 3.0_f64 * t462 * t5106 - t462 * t5110 / 3.0_f64 + 2.0_f64 * t92 * t5114 - t92 * t5118;
    (t5118, t5120)
}
