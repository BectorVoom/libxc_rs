//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1002/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1002(t102: f64, t652: f64, t2268: f64, t47: f64, t34: f64, t543: f64, t2854: f64, t52: f64, t538: f64, t6325: f64, t88: f64, t1859: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t21989 = t652 * t102;
    let t22014 = 1.0_f64 / t47 / t2268;
    let t22026 = t34 * t543;
    let t22034 = 1.0_f64 / t52 / t2854;
    let t22073 = t538 * t6325 * t88;
    let t22074 = 1920.0_f64 * t22073;
    let t22075 = t1859 * t1859;
    (t21989, t22014, t22026, t22034, t22074, t22075)
}
