//! GGA_C_GAPC lxc pol — lxc_pol part 35 (v4rho2sigma2_14) CSE chunk 844/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part35_v4rho2sigma2_14_chunk844(t3288: f64, t7708: f64, t9826: f64, t2778: f64, t9760: f64, t325: f64, t8998: f64, t129: f64, t8117: f64, t3337: f64, t8769: f64, t916: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9827 = t3288 * t7708;
    let t9828 = t9826 * t9827;
    let t9830 = t9760 * t2778;
    let t9832 = t325 * t8998;
    let t9833 = t9832 * t2778;
    let t9835 = t8117 * t129;
    let t9836 = t9835 * t3337;
    let t9838 = t916 * t8769;
    (t9827, t9828, t9830, t9832, t9833, t9836, t9838)
}
