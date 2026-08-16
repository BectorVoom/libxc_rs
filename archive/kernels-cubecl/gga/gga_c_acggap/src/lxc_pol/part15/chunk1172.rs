//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1172/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1172<F: Float>(t1734: F, t7380: F, t7381: F, t1886: F, t7605: F, t2041: F, t5598: F, t6167: F, t1817: F, t31863: F, t1896: F, t7614: F) -> (F, F, F, F, F, F) {
    let t40295 = t7380 * t7381 * t1734;
    let t40297 = t7605 * t1886;
    let t40299 = t2041 * t5598;
    let t40301 = t2041 * t6167;
    let t40308 = t31863 * t1817;
    let t40310 = t7614 * t1896;
    (t40295, t40297, t40299, t40301, t40308, t40310)
}
