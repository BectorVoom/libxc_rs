//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1325/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1325<F: Float>(t34267: F, t9660: F, t1772: F, t4823: F, t7201: F, t15870: F, t34104: F, t32909: F, t34122: F, t32942: F, t34045: F, t34192: F, t32990: F, t32955: F, t34125: F, t112240: F, t112420: F, t116520: F, t32948: F, t33005: F, t33056: F, t34073: F, t34218: F, t9652: F, t9922: F) -> (F, F) {
    let t117128 = 0.69444444444444444446e-2 * t34267 * t9660;
    let t117130 = t4823 * t7201 * t1772;
    let t117133 = t15870 * t34104;
    let t117136 = 0.69444444444444444446e-2 * t34122 * t32909;
    let t117138 = 0.69444444444444444446e-2 * t32942 * t34045;
    let t117140 = 0.26805555555555555556e-2 * t34192 * t32909;
    let t117146 = 0.69444444444444444446e-2 * t32990 * t34045;
    let t117153 = t34125 * t32955;
    let t117155 = -t117128 + 0.8041666666666666667e-2 * t117130 * t9652 + 0.55273148148148148147e-2 * t117133 + t117136 + t117138 + t117140 + 0.8041666666666666667e-2 * t112240 * t9922 + 0.40208333333333333335e-2 * t112420 * t9922 + t117146 - 0.26805555555555555556e-2 * t33056 * t116520 + 0.8041666666666666667e-2 * t32948 * t34218 - 0.20833333333333333334e-1 * t34073 * t33005 + 0.61728395061728395064e-2 * t117153;
    (t117133, t117155)
}
