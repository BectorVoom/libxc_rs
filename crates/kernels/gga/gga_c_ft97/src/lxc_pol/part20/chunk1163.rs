//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1163/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1163<F: Float>(t1882: F, t28330: F, t28309: F, t8392: F, t53891: F, t6074: F, t24737: F, t53798: F, t10007: F, t109681: F, t11593: F, t13672: F, t13702: F, t14116: F, t14171: F, t14182: F, t14183: F, t14187: F, t1449: F, t1901: F, t242: F, t24768: F, t28108: F, t28344: F, t28386: F, t3977: F, t446: F, t53797: F, t684: F, t729: F, t762: F, t97705: F, t97962: F, t97964: F, t97966: F, t97978: F) -> (F, F, F) {
    let t111068 = 2.0 / 27.0 * t1882 * t28330;
    let t111070 = 2.0 / 27.0 * t8392 * t28309;
    let t111085 = t53891 * t6074;
    let t111089 = t53798 * t24737;
    let t111100 = 4.0 / 9.0 * t53797 * t97705 * t14171 - 2.0 / 9.0 * t97962 + 16.0 / 27.0 * t97964 + 8.0 / 27.0 * t97966 - 2.0 / 9.0 * t1901 * t10007 * t28108 * t684 + t111068 - t111070 + 8.0 / 9.0 * t11593 * t14182 * t28386 * t14116 - 8.0 / 27.0 * t11593 * t14187 * t28344 * t14116 - 2.0 / 9.0 * t97978 + t446 * t729 * t762 * t1449 * t13672 / 3.0 + 8.0 / 9.0 * t53797 * t111085 * t13702 + 4.0 / 9.0 * t53797 * t111089 * t14183 + 2.0 / 3.0 * t446 * t729 * t3977 * t24768 - 2.0 / 3.0 * t446 * t242 * t109681;
    (t111085, t111089, t111100)
}
