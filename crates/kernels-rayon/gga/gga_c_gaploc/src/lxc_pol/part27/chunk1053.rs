//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1053/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1053(t4803: f64, t6582: f64, t1535: f64, t9419: f64, t1433: f64, t20395: f64, t2366: f64, t6519: f64, t9439: f64, t9448: f64, t1359: f64, t2293: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t20675 = t4803 * t6582;
    let t20687 = t1535 * t9419;
    let t20688 = t1433 * t20687;
    let t20692 = t2366 * t20395;
    let t20696 = t9439 * t6519;
    let t20700 = t9448 * t6519;
    let t20731 = t1359 * t2293;
    (t20675, t20687, t20688, t20692, t20696, t20700, t20731)
}
