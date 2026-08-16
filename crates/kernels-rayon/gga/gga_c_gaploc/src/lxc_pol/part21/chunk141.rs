//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 141/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk141(t177: f64, t501: f64, t178: f64, t400: f64, t108: f64, t75: f64, t14: f64, t1: f64, t112: f64, t3: f64, t78: f64, t110: f64, t72: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t502 = t177 * t501;
    let t503 = t400 * t178;
    let t506 = t75 * t108;
    let t507 = t506 * t14;
    let t508 = t112 * t1;
    let t509 = t3 * t78;
    let t510 = t508 * t509;
    let t513 = t110 * t72;
    (t502, t503, t506, t507, t508, t509, t510, t513)
}
