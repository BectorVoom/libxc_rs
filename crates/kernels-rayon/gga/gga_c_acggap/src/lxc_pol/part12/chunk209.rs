//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 209/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk209(t195: f64, t288: f64, t656: f64, t19: f64, t355: f64, t20: f64, t5: f64, t351: f64, t123: f64, t203: f64, t202: f64, t6: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t657 = t195 * t288;
    let t658 = t656 * t657;
    let t659 = 0.10843581300301739842e-1_f64 * t658;
    let t660 = t355 * t19;
    let t661 = t20 * t5;
    let t662 = t661 * t351;
    let t663 = t660 * t662;
    let t665 = t203 * t123;
    let t666 = t202 * t665;
    let t668 = t6 * t123;
    (t657, t659, t660, t661, t662, t663, t665, t666, t668)
}
