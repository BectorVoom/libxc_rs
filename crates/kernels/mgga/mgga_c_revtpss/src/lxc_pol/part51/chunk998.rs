//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 998/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk998<F: Float>(t126125: F, t32469: F, t33698: F, t686: F, t72: F, t119982: F, t119837: F, t14686: F, t1559: F, t120011: F, t120016: F, t1544: F, t886: F, t119792: F, t828: F, t855: F) -> (F, F, F, F, F, F, F) {
    let t126126 = t32469 * t126125;
    let t126129 = t33698 * t72 * t686;
    let t126130 = t119982 * t126129;
    let t126133 = t14686 * t119837 * t1559;
    let t126134 = t120011 * t126133;
    let t126136 = t120016 * t126133;
    let t126138 = t1544 * t886;
    let t126141 = t119792 * t855 * t828 * t126138;
    (t126126, t126129, t126130, t126134, t126136, t126138, t126141)
}
