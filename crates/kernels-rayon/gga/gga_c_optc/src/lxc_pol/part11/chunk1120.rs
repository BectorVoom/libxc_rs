//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1120/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1120(t16295: f64, t732: f64, t16247: f64, t40: f64, t591: f64, t16248: f64, t539: f64, t544: f64, t1: f64, t598: f64, t193: f64, t39009: f64, t4752: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t47765 = t732 * t16295;
    let t47871 = t40 * t16247 * t591;
    let t47877 = t539 * t16248;
    let t47879 = t544 * t16248;
    let t47886 = t16247 * t1 * t598;
    let t47896 = t193 * t39009 * t4752;
    (t47765, t47871, t47877, t47879, t47886, t47896)
}
