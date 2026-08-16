//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 973/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk973(t13406: f64, t38: f64, t4573: f64, t7771: f64, t2033: f64, t4579: f64, t7780: f64, t2040: f64, t13335: f64, t3431: f64, t3472: f64, t3477: f64, t581: f64, t608: f64, t612: f64) -> (f64, f64) {
    let t13407 = t38 * t13406;
    let t13422 = t7771 * t4573;
    let t13427 = t2033 * t4579;
    let t13432 = t7780 * t4573;
    let t13437 = t2040 * t4579;
    let t13442 = -280.0_f64 / 27.0_f64 * t13422 * t581 + 56.0_f64 / 9.0_f64 * t3472 * t3431 + 28.0_f64 / 9.0_f64 * t13427 * t581 - 4.0_f64 / 3.0_f64 * t608 * t13335 + 280.0_f64 / 27.0_f64 * t13432 * t581 + 56.0_f64 / 9.0_f64 * t3477 * t3431 + 28.0_f64 / 9.0_f64 * t13437 * t581 + 4.0_f64 / 3.0_f64 * t612 * t13335;
    (t13407, t13442)
}
