//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1405/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1405<F: Float>(t123: F, t2465: F, t886: F, t9291: F, t10982: F, t860: F, t9646: F, t2434: F, t2828: F, t10115: F, t251: F, t887: F) -> (F, F, F, F) {
    let t41102 = t2465 * t123 * t9291 * t886;
    let t41105 = t9646 * t860 * t10982;
    let t41115 = t2465 * t123 * t2434 * t2828;
    let t41117 = t10115 * t251;
    let t41118 = t41117 * t887;
    (t41102, t41105, t41115, t41118)
}
