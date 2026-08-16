//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1139/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1139<F: Float>(t1839: F, t322: F, t1181: F, t599: F, t7346: F, t39499: F, t301: F, t7337: F, t1859: F, t372: F, t7351: F, t7575: F) -> (F, F, F, F, F, F) {
    let t39743 = t1839 * t322;
    let t39746 = t7346 * t1181 * t599 * t39743;
    let t39750 = t7346 * t1181 * t599 * t39499;
    let t39753 = t1839 * t301;
    let t39756 = t7337 * t1181 * t599 * t39753;
    let t39763 = t7575 * t1181 * t7351 * t1859 * t372;
    (t39743, t39746, t39750, t39753, t39756, t39763)
}
