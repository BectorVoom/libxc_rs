//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 450/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk450(t4589: f64, t488: f64, t83: f64, t3238: f64, t979: f64, t452: f64, t942: f64, t986: f64, t110: f64, t4495: f64, t920: f64, t1903: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4590 = t488 * t4589;
    let t4591 = t83 * t4590;
    let t4594 = t3238 * t979;
    let t4595 = t83 * t4594;
    let t4599 = t452 * t986 * t942;
    let t4603 = t452 * t110 * t4495;
    let t4606 = t920 * t942;
    let t4607 = t1903 * t4606;
    (t4590, t4591, t4594, t4595, t4599, t4603, t4607)
}
