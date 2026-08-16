//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 2013/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2013(t99009: f64, t99011: f64, t99013: f64, t99019: f64, t99021: f64, t99023: f64, t99026: f64, t99029: f64, t99033: f64, t99035: f64, t99015: f64, t99017: f64, t99031: f64) -> f64 {
    let t103285 = 0.90702367218671976884e-1_f64 * t99009;
    let t103286 = 0.32012600194825403606e-1_f64 * t99011;
    let t103287 = 0.2168320119862840671e-2_f64 * t99013;
    let t103290 = 0.4065600224742826258e-3_f64 * t99019;
    let t103291 = 0.10164000561857065645e-3_f64 * t99021;
    let t103292 = 0.32012600194825403606e-1_f64 * t99023;
    let t103293 = 0.22866142996303859718e-3_f64 * t99026;
    let t103294 = 0.57165357490759649296e-4_f64 * t99029;
    let t103296 = 0.80031500487063509014e-2_f64 * t99033;
    let t103297 = 0.22675591804667994221e-1_f64 * t99035;
    let t103298 = -t103285 + t103286 + t103287 + 0.34299214494455789578e-2_f64 * t99015 - 0.17149607247227894789e-2_f64 * t99017 + t103290 - t103291 - t103292 - t103293 + t103294 - 0.10289764348336736873e0_f64 * t99031 + t103296 - t103297;
    t103298
}
