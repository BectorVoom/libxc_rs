//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 859/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk859(t297: f64, t8196: f64, t8195: f64, t123: f64, t2672: f64, t2606: f64, t8185: f64, t2747: f64, t282: f64, t8193: f64, t7380: f64, t935: f64, sigma0: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8197 = t8196 * t297;
    let t8198 = t8195 * t8197;
    let t8201 = t2672 * t123;
    let t8202 = t8201 * t2606;
    let t8203 = t8185 * t8202;
    let t8206 = t2747 * sigma0;
    let t8207 = t8206 * t282;
    let t8208 = t8207 * t8193;
    let t8209 = t7380 * t935;
    (t8197, t8198, t8201, t8203, t8206, t8207, t8208, t8209)
}
