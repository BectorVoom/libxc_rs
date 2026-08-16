//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1870/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1870(t27641: f64, t73: f64, t4975: f64, t988: f64, t4976: f64, t27418: f64, t994: f64) -> (f64, f64, f64, f64, f64) {
    let t27651 = t27641 * t73;
    let t27652 = t4975 * t988;
    let t27653 = t27651 * t27652;
    let t27656 = t27651 * t4976;
    let t27661 = t994 * t27418;
    (t27651, t27652, t27653, t27656, t27661)
}
