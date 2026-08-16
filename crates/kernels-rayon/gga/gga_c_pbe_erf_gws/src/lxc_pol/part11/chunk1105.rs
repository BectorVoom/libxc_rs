//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1105/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1105(t12544: f64, t30876: f64, t12583: f64, t1620: f64, t1621: f64, t25081: f64, t12532: f64, t7527: f64, t3454: f64, t16532: f64, t185: f64, t186: f64) -> (f64, f64, f64, f64) {
    let t47695 = 16.0_f64 / 5.0_f64 * t30876 * t12544;
    let t47699 = 32.0_f64 / 5.0_f64 * t1620 * t1621 * t25081 * t12583;
    let t47701 = 32.0_f64 / 5.0_f64 * t7527 * t12532;
    let t47702 = t3454 * t3454;
    let t47706 = 16.0_f64 / 5.0_f64 * t185 * t186 * t16532 * t47702;
    (t47695, t47699, t47701, t47706)
}
