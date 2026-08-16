//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 1009/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk1009(t12574: f64, t224: f64, t3899: f64, t987: f64, t3707: f64, t435: f64, t1736: f64, t474: f64, t177: f64, t208: f64, t4913: f64, t319: f64, t337: f64, t7061: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12575 = t224 * t12574;
    let t12664 = t987 * t3899;
    let t12744 = t435 * t3707;
    let t12768 = t474 * t1736;
    let t13281 = t177 / t4913 / t208;
    let t13296 = t319 / t7061 / t337;
    (t12575, t12664, t12744, t12768, t13281, t13296)
}
