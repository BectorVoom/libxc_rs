//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 425/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk425(t131: f64, t2078: f64, t130: f64, t142: f64, t654: f64, t661: f64, t103: f64, t137: f64) -> (f64, f64, f64, f64, f64) {
    let t2079 = t131 * t2078;
    let t2080 = t130 * t2079;
    let t2082 = 0.71839320644782096162e-1_f64 * t2080 * t142;
    let t2083 = t654 * t661;
    let t2085 = t137 * t103;
    let t2086 = 1.0_f64 / t2085;
    (t2079, t2080, t2082, t2083, t2086)
}
