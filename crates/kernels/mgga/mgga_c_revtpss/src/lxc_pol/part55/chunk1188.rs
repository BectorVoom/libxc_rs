//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1188/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1188<F: Float>(t31845: F, t33695: F, t839: F, t119837: F, t14686: F, t1559: F, t120011: F, t120016: F, t1544: F, t886: F, t119792: F, t828: F, t855: F) -> (F, F, F, F, F) {
    let t126121 = t33695 * t31845;
    let t126122 = t126121 * t839;
    let t126133 = t14686 * t119837 * t1559;
    let t126134 = t120011 * t126133;
    let t126136 = t120016 * t126133;
    let t126138 = t1544 * t886;
    let t126141 = t119792 * t855 * t828 * t126138;
    (t126122, t126134, t126136, t126138, t126141)
}
