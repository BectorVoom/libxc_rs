//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1007/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1007<F: Float>(t2068: F, t8480: F, t8778: F, t1181: F, t26214: F, t604: F, t30543: F, t9653: F, t5532: F, t7564: F, t8600: F, t2288: F, t5720: F, t15386: F, t31195: F, t13287: F, t2297: F, t5616: F) -> (F, F, F, F, F, F, F) {
    let t39879 = t2068 * t8480 * t8778;
    let t39883 = t2068 * t1181 * t604 * t26214;
    let t39885 = t30543 * t9653;
    let t39889 = t7564 * t1181 * t8600 * t5532;
    let t39891 = t2288 * t5720;
    let t39893 = t31195 * t15386 * t39891;
    let t39897 = t31195 * t13287 * t2297 * t5616;
    (t39879, t39883, t39885, t39889, t39891, t39893, t39897)
}
