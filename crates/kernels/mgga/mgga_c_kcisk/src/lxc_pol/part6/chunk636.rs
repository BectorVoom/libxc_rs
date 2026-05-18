//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 636/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk636<F: Float>(t5015: F, t8797: F, t5020: F, t7715: F, t1775: F, t1776: F, t7718: F, t5007: F, t5006: F, t2464: F, t5031: F, t1310: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t8798 = t5015 * t8797;
    let t8801 = t5020 * t7715;
    let t8802 = t1775 * t8801;
    let t8806 = t1776 * t7718;
    let t8807 = t1775 * t8806;
    let t8810 = t5007 * t7715;
    let t8811 = t5006 * t8810;
    let t8814 = t2464 * t2464;
    let t8815 = t5031 * t8814;
    let t8816 = t1310 * t8815;
    (t8798, t8801, t8802, t8806, t8807, t8810, t8811, t8814, t8815, t8816)
}
