//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1174/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1174<F: Float>(t120016: F, t126133: F, t1544: F, t886: F, t119792: F, t828: F, t855: F, t31753: F, t4435: F, t8478: F, t8484: F, t817: F, t8485: F, t98848: F) -> (F, F, F, F, F) {
    let t126136 = t120016 * t126133;
    let t126138 = t1544 * t886;
    let t126141 = t119792 * t855 * t828 * t126138;
    let t126145 = t8478 * t8484 * t31753 * t4435;
    let t126148 = t98848 * t8485 * t817;
    (t126136, t126138, t126141, t126145, t126148)
}
