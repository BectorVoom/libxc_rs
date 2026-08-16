//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 686/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk686(t1784: f64, t1790: f64, t1792: f64, t533: f64, t587: f64, t6407: f64, t1859: f64, t588: f64, t534: f64, t6434: f64, t1785: f64, t1835: f64, t209: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6465 = 0.48245472966453314466e2_f64 * t1790 * t1784 * t1792 * t533;
    let t6466 = t6407 * t587;
    let t6472 = t588 * t1859;
    let t6475 = t6434 * t534;
    let t6477 = 6.0_f64 * t1790 * t6475;
    let t6480 = 0.53425e-1_f64 * t209 * t1835 * t1785;
    (t6465, t6466, t6472, t6475, t6477, t6480)
}
