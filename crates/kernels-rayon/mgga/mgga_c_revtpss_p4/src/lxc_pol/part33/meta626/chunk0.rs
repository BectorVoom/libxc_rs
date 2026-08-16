//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2068/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2068(t99011: f64, t4349: f64, t93072: f64, t14673: f64, t92955: f64, t14688: f64, t4452: f64, t92951: f64, t14719: f64, t25227: f64, t2661: f64, t14723: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t99012 = 0.16006300097412701803e-1_f64 * t99011;
    let t99013 = t93072 * t4349;
    let t99019 = t92955 * t14673;
    let t99020 = 0.2032800112371413129e-3_f64 * t99019;
    let t99021 = t92955 * t14688;
    let t99022 = 0.50820002809285328226e-4_f64 * t99021;
    let t99023 = t92951 * t4452;
    let t99024 = 0.16006300097412701803e-1_f64 * t99023;
    let t99026 = t2661 * t25227 * t14719;
    let t99027 = 0.11433071498151929859e-3_f64 * t99026;
    let t99029 = t2661 * t25227 * t14723;
    (t99012, t99013, t99020, t99022, t99024, t99027, t99029)
}
