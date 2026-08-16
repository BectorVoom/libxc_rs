//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1924/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1924(t98968: f64, t98972: f64, t98983: f64, t98991: f64, t99000: f64, t99006: f64, t99011: f64, t99019: f64, t99021: f64, t99023: f64, t99026: f64, t99029: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t103265 = 0.11433071498151929859e-2_f64 * t98968;
    let t103267 = 0.4065600224742826258e-3_f64 * t98972;
    let t103273 = 0.4065600224742826258e-3_f64 * t98983;
    let t103276 = 0.80031500487063509014e-2_f64 * t98991;
    let t103280 = 0.22866142996303859718e-3_f64 * t99000;
    let t103283 = 0.57165357490759649296e-4_f64 * t99006;
    let t103286 = 0.32012600194825403606e-1_f64 * t99011;
    let t103290 = 0.4065600224742826258e-3_f64 * t99019;
    let t103291 = 0.10164000561857065645e-3_f64 * t99021;
    let t103292 = 0.32012600194825403606e-1_f64 * t99023;
    let t103293 = 0.22866142996303859718e-3_f64 * t99026;
    let t103294 = 0.57165357490759649296e-4_f64 * t99029;
    (t103265, t103267, t103273, t103276, t103280, t103283, t103286, t103290, t103291, t103292, t103293, t103294)
}
