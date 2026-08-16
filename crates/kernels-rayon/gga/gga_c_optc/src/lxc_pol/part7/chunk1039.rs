//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1039/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1039(t22643: f64, t1874: f64, t2042: f64, t1963: f64, t2048: f64, t1864: f64, t1867: f64, t22075: f64, t601: f64, t22403: f64, t22406: f64, t22410: f64, t22632: f64, t22634: f64, t22636: f64, t22638: f64, t22641: f64) -> (f64, f64, f64, f64, f64) {
    let t22644 = 0.73246220147012639764e-3_f64 * t22643;
    let t22645 = t2042 * t1874;
    let t22646 = 240.0_f64 * t22645;
    let t22647 = t2048 * t1963;
    let t22648 = 192.0_f64 * t22647;
    let t22652 = 0.51947267698127589897e2_f64 * t601 * t1864 * t22075 * t1867;
    let t22653 = t22632 - t22634 - t22403 + t22636 + t22638 - t22641 - t22644 - t22406 - t22410 + t22646 - t22648 - t22652;
    (t22644, t22646, t22648, t22652, t22653)
}
