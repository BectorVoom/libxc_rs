//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 415/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk415(t183: f64, t2211: f64, t2213: f64, t2218: f64, t2219: f64, t2224: f64, t724: f64, t727: f64, t102: f64, t108: f64, t176: f64) -> (f64, f64) {
    let t2226 = t2211 * t183 - 2.0_f64 * t2213 * t727 + 2.0_f64 * t2218 * t2219 - t724 * t2224;
    let t2227 = t2226 * t102;
    let t2229 = t176 * t2227 * t108;
    (t2226, t2229)
}
