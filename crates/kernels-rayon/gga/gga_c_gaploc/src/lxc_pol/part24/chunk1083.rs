//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1083/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1083(t540: f64, t8071: f64, t20550: f64, t7892: f64, t1: f64, t106: f64, t192: f64, t7861: f64, t1564: f64, t7905: f64, t9448: f64, t1397: f64, t8247: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t26279 = t8071 * t540;
    let t26328 = t20550 * t7892;
    let t26343 = t7861 * t1 * t106 * t192;
    let t26428 = t1564 * t7861;
    let t26435 = t9448 * t7905;
    let t26451 = t1397 * t8247;
    (t26279, t26328, t26343, t26428, t26435, t26451)
}
