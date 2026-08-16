//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1173/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1173<F: Float>(t126121: F, t839: F, t33707: F, t686: F, t72: F, t32469: F, t33698: F, t119982: F, t119837: F, t14686: F, t1559: F, t120011: F) -> (F, F, F, F, F, F, F) {
    let t126122 = t126121 * t839;
    let t126125 = t33707 * t72 * t686;
    let t126126 = t32469 * t126125;
    let t126129 = t33698 * t72 * t686;
    let t126130 = t119982 * t126129;
    let t126133 = t14686 * t119837 * t1559;
    let t126134 = t120011 * t126133;
    (t126122, t126125, t126126, t126129, t126130, t126133, t126134)
}
