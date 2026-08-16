//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 2010/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2010(t98937: f64, t98949: f64, t92952: f64, t92956: f64, t98940: f64, t98943: f64, t98945: f64, t98947: f64, t98951: f64, t98953: f64, t98955: f64, t98957: f64) -> f64 {
    let t103247 = 0.16006300097412701803e-1_f64 * t98937;
    let t103254 = 0.32012600194825403606e-1_f64 * t98949;
    let t103259 = -t103247 - 0.32012600194825403606e-1_f64 * t92952 - 0.85748036236139473944e-3_f64 * t98940 + 0.4065600224742826258e-3_f64 * t92956 - 0.17149607247227894789e-1_f64 * t98943 + 0.34299214494455789578e-2_f64 * t98945 - 0.68598428988911579156e-2_f64 * t98947 - t103254 - 0.34299214494455789578e-1_f64 * t98951 - 0.85748036236139473944e-3_f64 * t98953 - 0.13719685797782315831e-1_f64 * t98955 + 0.68598428988911579156e-2_f64 * t98957;
    t103259
}
