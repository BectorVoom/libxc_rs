//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 851/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk851<F: Float>(t157: F, t1734: F, t406: F, t1487: F, t524: F, t1795: F, t1410: F, t1748: F, t1854: F, t322: F, t7158: F, t372: F) -> (F, F, F, F, F, F, F) {
    let t25941 = t1734 * t406 * t157;
    let t26108 = t1487 * t524 * t157;
    let t26214 = t1795 * t406 * t157;
    let t26459 = t1748 * t1410;
    let t26554 = t1854 * t322;
    let t26757 = t7158 * t406;
    let t26956 = t1854 * t372;
    (t25941, t26108, t26214, t26459, t26554, t26757, t26956)
}
