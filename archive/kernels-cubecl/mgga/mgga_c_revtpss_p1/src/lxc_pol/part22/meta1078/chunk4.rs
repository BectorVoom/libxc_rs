//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3864/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3864<F: Float>(t22169: F, t46691: F, t22173: F, t9744: F, t6856: F, t9779: F, t6880: F, t22062: F, t9775: F, t13845: F, t22145: F, t48100: F) -> (F, F, F, F, F, F) {
    let t74269 = t46691 * t22169;
    let t74271 = t9744 * t22173;
    let t74277 = t9779 * t6856;
    let t74279 = t9779 * t6880;
    let t74281 = t9775 * t22062;
    let t74288 = t13845 * t48100 * t22145;
    (t74269, t74271, t74277, t74279, t74281, t74288)
}
