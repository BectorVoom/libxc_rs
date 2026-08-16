//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 465/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk465(t2289: f64, t45: f64, t631: f64, t78: f64, t57: f64, t635: f64, t81: f64, t112: f64, t111: f64, t654: f64, t99: f64, t107: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2290 = 88.0_f64 / 9.0_f64 * t2289;
    let t2297 = t631 * t45;
    let t2299 = 1.0_f64 / t78 / t2297;
    let t2304 = t635 * t57;
    let t2306 = 1.0_f64 / t81 / t2304;
    let t2335 = 11.0_f64 / 9.0_f64 * t2289 * t112;
    let t2339 = 1.0_f64 / t654 / t111;
    let t2349 = 1.0_f64 / t99;
    let t2357 = 1.0_f64 / t107;
    (t2290, t2297, t2299, t2304, t2306, t2335, t2339, t2349, t2357)
}
