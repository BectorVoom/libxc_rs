//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 700/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk700(t6646: f64, t1956: f64, t732: f64, t103: f64, t193: f64, t197: f64, t652: f64, t102: f64, t133: f64, t751: f64, t1928: f64, t745: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6647 = 0.51947267698127589899e2_f64 * t6646;
    let t6648 = t732 * t1956;
    let t6653 = 15400.0_f64 / 243.0_f64 * t193 * t652 * t103 * t197;
    let t6654 = t133 * t102;
    let t6656 = t193 * t6654 * t751;
    let t6660 = t193 * t745 * t1928 * t197;
    (t6647, t6648, t6653, t6654, t6656, t6660)
}
