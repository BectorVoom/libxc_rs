//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 811/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk811(t22634: f64, t2684: f64, t9438: f64, t2033: f64, t2365: f64, t40586: f64, t28924: f64, t6111: f64, t12656: f64, t22665: f64, t7427: f64, t29285: f64) -> (f64, f64, f64, f64, f64) {
    let t41448 = t2684 * t9438 * t22634;
    let t41451 = t2033 * t2365 * t40586;
    let t41454 = t6111 * t2365 * t28924;
    let t41457 = t7427 * t22665 * t12656;
    let t41460 = t6111 * t2365 * t29285;
    (t41448, t41451, t41454, t41457, t41460)
}
