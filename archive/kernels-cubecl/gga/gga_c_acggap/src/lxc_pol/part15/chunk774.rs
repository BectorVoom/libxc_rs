//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 774/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk774<F: Float>(t532: F, t7605: F, t1569: F, t2001: F, t1967: F, t2327: F, t5616: F, t604: F, t1181: F, t2068: F, t7380: F, t8544: F) -> (F, F, F, F, F, F) {
    let t8718 = t7605 * t532;
    let t8720 = t2001 * t1569;
    let t8722 = t1967 * t2327;
    let t8738 = t604 * t5616;
    let t8739 = t1181 * t8738;
    let t8740 = t2068 * t8739;
    let t8742 = t7380 * t8544;
    (t8718, t8720, t8722, t8739, t8740, t8742)
}
