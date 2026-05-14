//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1259/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1259<F: Float>(t410: F, t9014: F, t1823: F, t3142: F, t1859: F, t1862: F, t8589: F, t741: F, t8967: F, t750: F, t1861: F, t8892: F, t1860: F, t1726: F, t1727: F, t3129: F) -> (F, F, F, F, F, F, F, F) {
    let t28815 = t410 * t9014;
    let t28833 = t3142 * t1823;
    let t28836 = t1859 * t8589 * t1862;
    let t28838 = t8967 * t741;
    let t28840 = t8967 * t750;
    let t28842 = t8892 * t1861;
    let t28843 = t1860 * t28842;
    let t28846 = t1726 * t3129 * t1727;
    (t28815, t28833, t28836, t28838, t28840, t28842, t28843, t28846)
}
