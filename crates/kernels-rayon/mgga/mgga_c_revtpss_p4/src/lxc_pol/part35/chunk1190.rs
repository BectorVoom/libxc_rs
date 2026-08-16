//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1190/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1190(t22829: f64, t26028: f64, t27932: f64, t85776: f64, t22890: f64, t22849: f64, t7252: f64, t22877: f64, t94516: f64, t22881: f64, t22895: f64, t22837: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t114521 = t26028 * t22829;
    let t114525 = t27932 * t85776;
    let t114527 = t26028 * t22890;
    let t114541 = t7252 * t22849;
    let t114543 = t94516 * t22877;
    let t114545 = t26028 * t22881;
    let t114547 = t26028 * t22895;
    let t114549 = t26028 * t22837;
    (t114521, t114525, t114527, t114541, t114543, t114545, t114547, t114549)
}
