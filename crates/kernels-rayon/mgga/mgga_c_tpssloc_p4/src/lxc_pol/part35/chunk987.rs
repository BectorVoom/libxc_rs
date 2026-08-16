//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 987/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk987(t20936: f64, t225: f64, t237: f64, t119: f64, t20756: f64, t210: f64, t1484: f64, t5544: f64) -> (f64, f64, f64, f64) {
    let t20937 = t20936 * t225;
    let t20938 = t20937 * t237;
    let t20943 = t119 * t20756;
    let t20944 = t210 * t20943;
    let t20947 = t1484 * t5544;
    (t20937, t20938, t20944, t20947)
}
