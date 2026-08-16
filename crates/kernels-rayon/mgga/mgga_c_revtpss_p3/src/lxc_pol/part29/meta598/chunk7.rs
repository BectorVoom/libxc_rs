//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 2034/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2034(t100944: f64, t100947: f64, t100953: f64, t100958: f64, t100969: f64, t100978: f64, t101029: f64, t101032: f64, t101086: f64, t102851: f64, t102858: f64, t1711: f64, t1940: f64, t26425: f64, t26581: f64, t27793: f64, t27800: f64, t28291: f64, t3351: f64, t7432: f64, t8020: f64, t95511: f64) -> f64 {
    let t103750 = 6.0_f64 * t26425 * t100978 + 2.0_f64 * t102851 * t27800 - 3.0_f64 * t26425 * t100944 + t1940 * t26581 * t1711 / 2.0_f64 - 3.0_f64 * t28291 * t100958 + 3.0_f64 * t26425 * t101086 + t1940 * t8020 * t3351 / 2.0_f64 - 3.0_f64 * t95511 * t27793 + 6.0_f64 * t28291 * t101029 + 6.0_f64 * t28291 * t101032 - 3.0_f64 / 2.0_f64 * t26425 * t100947 - t1940 * t7432 * t100969 / 2.0_f64 - t102858 - 6.0_f64 * t28291 * t100953;
    t103750
}
