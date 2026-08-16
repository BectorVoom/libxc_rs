//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 485/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk485(t2310: f64, t598: f64, t495: f64, t599: f64, t142: f64, t2030: f64, t513: f64, t604: f64, t2060: f64, t2001: f64, t537: f64, t542: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2311 = t598 * t2310;
    let t2313 = t599 * t495;
    let t2314 = t142 * t2313;
    let t2315 = t2030 * t2314;
    let t2317 = t604 * t513;
    let t2318 = t142 * t2317;
    let t2319 = t2060 * t2318;
    let t2321 = t2001 * t537;
    let t2323 = t2001 * t542;
    (t2311, t2313, t2314, t2315, t2317, t2318, t2319, t2321, t2323)
}
