//! GGA_C_GAPC lxc pol — lxc_pol part 32 (v4rho2sigma2_11) CSE chunk 846/1311 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part32_v4rho2sigma2_11_chunk846(t2660: f64, t8624: f64, t7330: f64, t7335: f64, t9810: f64, t1077: f64, t2713: f64, t3307: f64, t910: f64, t1069: f64, t2508: f64, t191: f64, t2674: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9815 = t2660 * t8624;
    let t9816 = t9815 * t7330;
    let t9818 = t9810 * t7335;
    let t9820 = t1077 * t2713;
    let t9822 = t3307 * t910;
    let t9824 = t1069 * t2508;
    let t9826 = t2674 * t191;
    (t9816, t9818, t9820, t9822, t9824, t9826)
}
