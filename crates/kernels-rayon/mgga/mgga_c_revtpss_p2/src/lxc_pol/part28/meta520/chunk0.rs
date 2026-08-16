//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1942/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1942(t4975: f64, t988: f64, t27651: f64, t4976: f64, t27418: f64, t994: f64, t1096: f64, t27638: f64, t3143: f64, t1983: f64, t27642: f64, t4983: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t27652 = t4975 * t988;
    let t27653 = t27651 * t27652;
    let t27656 = t27651 * t4976;
    let t27661 = t994 * t27418;
    let t27664 = t4975 * t1096;
    let t27665 = t27651 * t27664;
    let t27668 = t27638 * t3143;
    let t27669 = t1983 * t27668;
    let t27670 = t27642 * t4983;
    (t27652, t27653, t27656, t27661, t27664, t27665, t27668, t27669, t27670)
}
