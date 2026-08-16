//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1172/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1172(t1734: f64, t7380: f64, t7381: f64, t1886: f64, t7605: f64, t2041: f64, t5598: f64, t6167: f64, t1817: f64, t31863: f64, t1896: f64, t7614: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t40295 = t7380 * t7381 * t1734;
    let t40297 = t7605 * t1886;
    let t40299 = t2041 * t5598;
    let t40301 = t2041 * t6167;
    let t40308 = t31863 * t1817;
    let t40310 = t7614 * t1896;
    (t40295, t40297, t40299, t40301, t40308, t40310)
}
