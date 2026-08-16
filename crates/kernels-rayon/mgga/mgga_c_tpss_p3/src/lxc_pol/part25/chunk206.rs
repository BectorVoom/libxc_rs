//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 206/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk206(t143: f64, t130: f64, t131: f64, t72: f64, t122: f64, t125: f64) -> (f64, f64, f64, f64, f64) {
    let t655 = t143 * t143;
    let t656 = 1.0_f64 / t655;
    let t657 = t130 * t656;
    let t659 = 1.0_f64 / t131 * t72;
    let t660 = t122 * t125;
    (t655, t656, t657, t659, t660)
}
