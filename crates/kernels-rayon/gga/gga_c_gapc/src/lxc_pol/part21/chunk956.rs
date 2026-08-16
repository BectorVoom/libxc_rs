//! GGA_C_GAPC lxc pol — lxc_pol part 21 (v4rho2sigma2_0) CSE chunk 956/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part21_v4rho2sigma2_0_chunk956(t19: f64, t2207: f64, t10346: f64, t11210: f64, t2580: f64, t11214: f64, t268: f64, t6853: f64, t6857: f64, t829: f64, t3235: f64, t3729: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11656 = t2207 * t19;
    let t11657 = t10346 * t11656;
    let t11658 = t11210 * t2580;
    let t11659 = t11657 * t11658;
    let t11661 = t11214 * t268;
    let t11662 = t11661 * t6853;
    let t11663 = t829 * t6857;
    let t11664 = t11662 * t11663;
    let t11666 = t3235 * t3729;
    (t11656, t11657, t11658, t11659, t11661, t11662, t11663, t11664, t11666)
}
