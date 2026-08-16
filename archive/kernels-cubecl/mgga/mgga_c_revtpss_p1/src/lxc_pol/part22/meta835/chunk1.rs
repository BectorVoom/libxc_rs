//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2961/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2961<F: Float>(t47247: F, t828: F, t13967: F, t9962: F, t13941: F, t46740: F, t221: F, t47273: F, t13785: F, t9816: F, t13770: F, t9775: F) -> (F, F, F, F, F, F) {
    let t48798 = t47247 * t828;
    let t48811 = t9962 * t13967;
    let t48813 = t46740 * t13941;
    let t48823 = t47273 * t221;
    let t48825 = t9816 * t48823 * t13785;
    let t48827 = t9775 * t13770;
    (t48798, t48811, t48813, t48823, t48825, t48827)
}
