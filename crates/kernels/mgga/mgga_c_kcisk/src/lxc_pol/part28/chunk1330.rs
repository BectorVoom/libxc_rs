//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1330/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1330<F: Float>(t32942: F, t34045: F, t32909: F, t34192: F, t32990: F, t32955: F, t34125: F, t116477: F, t33002: F, t34228: F, t9660: F, t34264: F, t5060: F, t654: F, t719: F, t34275: F) -> (F, F, F, F, F, F, F, F, F) {
    let t117138 = 0.69444444444444444446e-2 * t32942 * t34045;
    let t117140 = 0.26805555555555555556e-2 * t34192 * t32909;
    let t117146 = 0.69444444444444444446e-2 * t32990 * t34045;
    let t117153 = t34125 * t32955;
    let t117159 = 0.15520416666666666667e-2 * t33002 * t116477;
    let t117161 = 0.69444444444444444446e-2 * t34228 * t9660;
    let t117170 = 0.69444444444444444446e-2 * t34264 * t9660;
    let t117182 = t5060 * t654 * t719;
    let t117192 = 0.18518518518518518519e-1 * t34275 * t9660;
    (t117138, t117140, t117146, t117153, t117159, t117161, t117170, t117182, t117192)
}
