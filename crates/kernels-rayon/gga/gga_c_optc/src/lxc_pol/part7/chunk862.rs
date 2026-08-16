//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 862/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk862(t2760: f64, t888: f64, t2758: f64, t2753: f64, t2751: f64, t140: f64, t2665: f64, t883: f64, t2661: f64, t2748: f64, t7878: f64, t942: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8243 = t888 * t2760;
    let t8244 = t2758 * t8243;
    let t8246 = t888 * t2753;
    let t8247 = t2751 * t8246;
    let t8250 = t883 * t2665 * t140;
    let t8251 = t2661 * t8250;
    let t8254 = t2748 * t8250;
    let t8257 = t7878 * t942;
    (t8243, t8244, t8246, t8247, t8251, t8254, t8257)
}
