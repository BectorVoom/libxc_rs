//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1032/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1032<F: Float>(t1776: F, t22591: F, t1775: F, t10872: F, t8814: F, t1785: F, t7261: F, t4998: F, t8806: F, t1773: F, t10886: F, t8810: F, t8801: F, t7208: F, t7253: F, t1769: F, t8833: F) -> (F, F, F, F, F, F, F) {
    let t23790 = t1776 * t22591;
    let t23791 = t1775 * t23790;
    let t23796 = t10872 * t8814;
    let t23797 = t23796 * t1785;
    let t23798 = t7261 * t23797;
    let t23801 = t4998 * t8806;
    let t23802 = t1773 * t23801;
    let t23804 = t10886 * t8810;
    let t23805 = t1773 * t23804;
    let t23807 = t4998 * t8801;
    let t23808 = t1773 * t23807;
    let t23811 = t7208 * t7253;
    let t23814 = t8833 * t1769;
    (t23791, t23798, t23802, t23805, t23808, t23811, t23814)
}
