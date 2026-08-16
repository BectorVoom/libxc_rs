//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 429/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk429(t145: f64, t154: f64, t355: f64, t2035: f64, t1969: f64, t1971: f64, t1974: f64, t1987: f64, t1990: f64, t1996: f64, t2000: f64, t2002: f64, t2004: f64, t2006: f64, t2011: f64, t2014: f64, t2018: f64, t2022: f64, t2024: f64, t2026: f64, t2033: f64) -> (f64, f64, f64) {
    let t2037 = t154 * t355 * t145;
    let t2038 = t2035 * t2037;
    let t2039 = t2038 / 192.0_f64;
    let t2040 = t1969 - t1971 + 0.10718504529517434243e-3_f64 * t1974 + t1987 - t1990 - t1996 - t2000 - 0.17149607247227894789e-2_f64 * t2002 + 0.85748036236139473944e-3_f64 * t2004 - 0.85748036236139473944e-3_f64 * t2006 - t2011 + t2014 + t2018 - t2022 - t2024 + t2026 / 96.0_f64 + 0.114609375e-1_f64 * t2033 + t2039;
    (t2037, t2039, t2040)
}
