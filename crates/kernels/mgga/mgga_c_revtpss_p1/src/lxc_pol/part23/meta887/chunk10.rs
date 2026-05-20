//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2811/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2811<F: Float>(t212: F, t23359: F, t689: F, t780: F, t23177: F, t2798: F, t686: F, t72: F, t14568: F, t18730: F, t14586: F, t6016: F) -> (F, F, F, F) {
    let t76081 = t689 * t212 * t23359 * t780;
    let t76100 = t2798 * t23177 * t72 * t686;
    let t76104 = t14568 * t18730;
    let t76106 = t14586 * t6016;
    (t76081, t76100, t76104, t76106)
}
