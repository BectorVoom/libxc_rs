//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 869/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk869(t3764: f64, t377: f64, t409: f64, t3372: f64, t3445: f64, t329: f64, t3615: f64, t124: f64, t19: f64, t7335: f64, t1162: f64, t12309: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12531 = t377 * t3764;
    let t12532 = t12531 * t409;
    let t12536 = t3372 * t3445;
    let t12572 = t329 * t3615;
    let t12576 = t124 * t7335 * t19;
    let t12586 = t12309 * t1162;
    (t12531, t12532, t12536, t12572, t12576, t12586)
}
