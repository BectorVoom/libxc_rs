//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 934/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk934(t522: f64, t9212: f64, t9214: f64, t3824: f64, t592: f64, t11976: f64, t11978: f64, t11980: f64, t11982: f64, t11984: f64, t9457: f64, t9476: f64, t9484: f64, t9780: f64) -> (f64, f64, f64, f64) {
    let t12044 = 24.0_f64 * t9212 * t522;
    let t12045 = t9214 * t522;
    let t12046 = 144.0_f64 * t12045;
    let t12048 = 12.0_f64 * t592 * t3824;
    let t12049 = -t9457 + t9476 + t9484 + t11976 - t11978 - t11980 - t11982 - t11984 + t9780 + t12044 - t12046 - t12048;
    (t12044, t12046, t12048, t12049)
}
