//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1380/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1380(t2856: f64, t9057: f64, t27453: f64, t4435: f64, t9074: f64, t1179: f64, t2586: f64, t9193: f64, t8487: f64, t8914: f64, t3109: f64, t9175: f64) -> (f64, f64, f64, f64, f64) {
    let t27465 = t2856 * t9057;
    let t27470 = t4435 * t27453 * t9074;
    let t27473 = t1179 * t2586 * t9193;
    let t27481 = t8487 * t8914;
    let t27483 = t9175 * t27481 * t3109;
    (t27465, t27470, t27473, t27481, t27483)
}
