//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 747/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk747(t7802: f64, t7805: f64, t7849: f64, t7853: f64, t7862: f64, t394: f64, t633: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8276 = 0.31448092289604152069e-3_f64 * t7802;
    let t8278 = 0.41930789719472202758e-3_f64 * t7805;
    let t8291 = 77.0_f64 / 864.0_f64 * t7849;
    let t8292 = 35.0_f64 / 216.0_f64 * t7853;
    let t8294 = t7862 / 192.0_f64;
    let t8306 = t394 * t633;
    (t8276, t8278, t8291, t8292, t8294, t8306)
}
