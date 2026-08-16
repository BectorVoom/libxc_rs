//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 511/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk511(t300: f64, t3018: f64, t2980: f64, t960: f64, t983: f64, t2986: f64, t2988: f64, t973: f64, t981: f64, t3006: f64, t964: f64, t3011: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3019 = t300 * t3018;
    let t3021 = 0.19751673498613801407e-1_f64 * t300 * t2980;
    let t3022 = t300 * t960;
    let t3024 = 0.11696447245269292414e1_f64 * t3022 * t983;
    let t3026 = t2986 * t2988 * t973;
    let t3028 = 0.11696447245269292414e1_f64 * t981 * t3026;
    let t3030 = t964 * t3006 * t973;
    let t3032 = 0.5848223622634646207e0_f64 * t981 * t3030;
    let t3033 = t3011 * t2988;
    (t3019, t3021, t3022, t3024, t3026, t3028, t3030, t3032, t3033)
}
