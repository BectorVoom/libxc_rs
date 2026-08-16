//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 815/1444 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk815(t647: f64, t9306: f64, t2998: f64, t9305: f64, t2993: f64, t9282: f64, t3001: f64, t129: f64, t5987: f64, t2987: f64, t197: f64, t5799: f64) -> (f64, f64, f64, f64, f64) {
    let t9307 = t647 * t9306;
    let t9308 = t2998 * t9307;
    let t9309 = t9305 * t9308;
    let t9311 = t2993 * t9282;
    let t9312 = t9311 * t3001;
    let t9314 = t5987 * t129;
    let t9315 = t9314 * t2987;
    let t9317 = t197 * t5799;
    (t9308, t9309, t9312, t9315, t9317)
}
