//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 811/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk811(t8556: f64, t8574: f64, t8580: f64, t8582: f64, t8607: f64, t8619: f64, t8625: f64, t8650: f64, t8680: f64, t8682: f64, t8684: f64, t8690: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9206 = 0.10482697429868050689e-2_f64 * t8556;
    let t9211 = 0.85748036236139473944e-3_f64 * t8574;
    let t9214 = 0.18868855373762491241e-2_f64 * t8580;
    let t9215 = 0.21437009059034868486e-3_f64 * t8582;
    let t9222 = 0.42874018118069736972e-3_f64 * t8607;
    let t9226 = 0.28015625e-1_f64 * t8619;
    let t9228 = 7.0_f64 / 144.0_f64 * t8625;
    let t9239 = 0.10718504529517434243e-2_f64 * t8650;
    let t9248 = 11.0_f64 / 192.0_f64 * t8680;
    let t9249 = 11.0_f64 / 576.0_f64 * t8682;
    let t9250 = 7.0_f64 / 72.0_f64 * t8684;
    let t9252 = 0.21437009059034868486e-3_f64 * t8690;
    (t9206, t9211, t9214, t9215, t9222, t9226, t9228, t9239, t9248, t9249, t9250, t9252)
}
