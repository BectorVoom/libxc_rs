//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2774/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2774<F: Float>(t14767: F, t221: F, t10703: F, t2674: F, t2661: F, t2662: F, t2754: F, t4352: F, t14728: F, t9775: F, t1549: F, t40861: F) -> (F, F, F, F) {
    let t50931 = t221 * t14767;
    let t50933 = t2674 * t10703 * t50931;
    let t50937 = t2661 * t2662 * t4352 * t2754;
    let t50939 = t9775 * t14728;
    let t50941 = t40861 * t1549;
    (t50933, t50937, t50939, t50941)
}
