//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1218/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1218(t2606: f64, t2640: f64, t2644: f64, t7467: f64, t7471: f64, t7488: f64, t7906: f64, t861: f64, t24: f64, t7920: f64, t862: f64, t7925: f64) -> (f64, f64, f64, f64, f64) {
    let t25137 = t2640 * t7467 * t2606 * t2644;
    let t25145 = t7488 * t7471;
    let t25158 = t7906 * t861;
    let t25166 = t862 * t24 * t7920;
    let t25169 = t862 * t24 * t7925;
    (t25137, t25145, t25158, t25166, t25169)
}
