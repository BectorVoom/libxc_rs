//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2815/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2815<F: Float>(t23168: F, t39598: F, t686: F, t72: F, t10530: F, t23172: F, t1558: F, t231: F, t6016: F, t2782: F, t2797: F, t23167: F, t251: F) -> (F, F, F, F) {
    let t76153 = t39598 * t23168 * t72 * t686;
    let t76158 = t10530 * t23172 * t72 * t686;
    let t76161 = t6016 * t1558 * t231;
    let t76163 = t2782 * t2797 * t76161;
    let t76169 = t251 * t23167;
    (t76153, t76158, t76163, t76169)
}
