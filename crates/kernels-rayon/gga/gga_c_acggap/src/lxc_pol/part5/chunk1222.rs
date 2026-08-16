//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1222/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1222(t3431: f64, t5891: f64, t1165: f64, t3451: f64, t4183: f64, t5852: f64, t3372: f64, t6157: f64, t13092: f64, t5932: f64, t17550: f64, t5928: f64) -> (f64, f64, f64, f64, f64) {
    let t22349 = t3431 * t5891;
    let t22369 = t3451 * t1165 * t5852 * t4183;
    let t22371 = t3372 * t6157;
    let t22378 = t13092 * t5932;
    let t22380 = t17550 * t5928;
    (t22349, t22369, t22371, t22378, t22380)
}
