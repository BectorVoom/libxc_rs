//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 1074/1447 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk1074(t12628: f64, t12653: f64, t224: f64, t3916: f64, t987: f64, t3707: f64, t435: f64, t1736: f64, t474: f64, t177: f64, t208: f64, t4913: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12654 = t12628 + t12653;
    let t12655 = t224 * t12654;
    let t12667 = t987 * t3916;
    let t12744 = t435 * t3707;
    let t12768 = t474 * t1736;
    let t13281 = t177 / t4913 / t208;
    (t12654, t12655, t12667, t12744, t12768, t13281)
}
