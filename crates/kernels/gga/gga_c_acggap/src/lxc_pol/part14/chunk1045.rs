//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1045/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1045<F: Float>(t1817: F, t31863: F, t1896: F, t7614: F, t1866: F, t361: F, t7436: F, t142: F, t6304: F, t1998: F, t5971: F, t1426: F, t1894: F, t2085: F, t598: F, t1967: F, t9549: F) -> (F, F, F, F, F, F, F) {
    let t40308 = t31863 * t1817;
    let t40310 = t7614 * t1896;
    let t40313 = t7436 * t361 * t1866;
    let t40316 = t7436 * t142 * t6304;
    let t40318 = t1998 * t5971;
    let t40322 = t598 * t1426 * t1894 * t2085;
    let t40324 = t1967 * t9549;
    (t40308, t40310, t40313, t40316, t40318, t40322, t40324)
}
