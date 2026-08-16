//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 662/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk662(t583: f64, t8282: f64, t462: f64, t9178: f64, t9179: f64, t9181: f64, t9183: f64, t9186: f64, t9188: f64, t9190: f64, t9193: f64, t9196: f64, t9199: f64, t92: f64) -> f64 {
    let t9202 = t8282 * t583;
    let t9204 = -t9178 - 4.0_f64 / 3.0_f64 * t9179 + t462 * t9181 + t462 * t9183 - t92 * t9186 - 2.0_f64 / 3.0_f64 * t9188 - 2.0_f64 / 3.0_f64 * t9190 + 2.0_f64 / 3.0_f64 * t462 * t9193 + 4.0_f64 / 3.0_f64 * t462 * t9196 - 2.0_f64 / 3.0_f64 * t462 * t9199 - 4.0_f64 / 9.0_f64 * t9202;
    t9204
}
