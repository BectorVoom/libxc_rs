//! GGA_C_GAPC lxc pol — lxc_pol part 32 (v4rho2sigma2_11) CSE chunk 996/1311 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part32_v4rho2sigma2_11_chunk996(t12006: f64, t224: f64, t3797: f64, t987: f64, t3707: f64, t435: f64, t1736: f64, t474: f64, t177: f64, t208: f64, t4913: f64, t319: f64, t337: f64, t7061: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12007 = t224 * t12006;
    let t12658 = t987 * t3797;
    let t12744 = t435 * t3707;
    let t12768 = t474 * t1736;
    let t13281 = t177 / t4913 / t208;
    let t13296 = t319 / t7061 / t337;
    (t12007, t12658, t12744, t12768, t13281, t13296)
}
