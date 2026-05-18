//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1189/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1189<F: Float>(t31753: F, t4435: F, t8478: F, t8484: F, t817: F, t8485: F, t98848: F, t126078: F, t2747: F, t31767: F, t31772: F, t124: F, t1579: F, t800: F, t815: F) -> (F, F, F, F) {
    let t126145 = t8478 * t8484 * t31753 * t4435;
    let t126148 = t98848 * t8485 * t817;
    let t126158 = t31767 * t2747 * t31772 * t126078;
    let t126163 = t815 * t800 * t124 * t1579;
    (t126145, t126148, t126158, t126163)
}
