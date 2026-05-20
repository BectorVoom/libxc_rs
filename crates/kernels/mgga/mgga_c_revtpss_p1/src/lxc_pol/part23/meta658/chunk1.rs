//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2389/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2389<F: Float>(t10504: F, t138: F, t886: F, t9302: F, t123: F, t2465: F, t9291: F, t10982: F, t860: F, t9646: F, t10115: F, t251: F) -> (F, F, F, F) {
    let t41098 = t10504 * t138 * t9302 * t886;
    let t41102 = t2465 * t123 * t9291 * t886;
    let t41105 = t9646 * t860 * t10982;
    let t41117 = t10115 * t251;
    (t41098, t41102, t41105, t41117)
}
