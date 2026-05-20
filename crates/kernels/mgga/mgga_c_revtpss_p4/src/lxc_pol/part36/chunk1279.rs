//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1279/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1279<F: Float>(t30100: F, t689: F, t25904: F, t25899: F, t30031: F, t686: F, t72: F, t25878: F, t108278: F, t786: F, t7286: F, t27989: F, t97802: F) -> (F, F, F, F, F, F) {
    let t108334 = t30100 * t689;
    let t108335 = t25904 * t108334;
    let t108337 = t25899 * t108334;
    let t108368 = t30031 * t72 * t686;
    let t108369 = t25878 * t108368;
    let t108379 = t786 * t108278;
    let t108380 = t108379 * t7286;
    let t108389 = t97802 * t27989;
    (t108335, t108337, t108368, t108369, t108380, t108389)
}
