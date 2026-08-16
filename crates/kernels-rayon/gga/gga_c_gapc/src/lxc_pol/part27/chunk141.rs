//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 141/1310 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk141(t115: f64, t118: f64, t1: f64, t463: f64, t125: f64, t3: f64, t128: f64) -> (f64, f64, f64, f64, f64) {
    let t464 = t115 * t118;
    let t465 = t464 * t1;
    let t466 = t463 * t465;
    let t467 = t3 * t125;
    let t468 = t467 * t128;
    (t464, t465, t466, t467, t468)
}
