//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1116/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1116(t35315: f64, t4987: f64, t7647: f64, t4364: f64, t7822: f64, t4963: f64, t7561: f64, t2937: f64, t524: f64, t943: f64, t1165: f64, t30856: f64, t604: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t35316 = 0.64311027177104605458e-2_f64 * t35315;
    let t35317 = t7647 * t4987;
    let t35318 = 0.17149607247227894789e-2_f64 * t35317;
    let t35319 = t7822 * t4364;
    let t35321 = t7561 * t4963;
    let t35324 = t524 * t2937 * t943;
    let t35327 = t30856 * t1165 * t604 * t35324;
    (t35316, t35318, t35319, t35321, t35324, t35327)
}
