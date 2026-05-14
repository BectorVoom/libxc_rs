//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1003/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1003<F: Float>(t9793: F, t9794: F, t9798: F, t9799: F, t9802: F, t9810: F, t9818: F, t9829: F, t354: F, t2999: F, t889: F) -> (F, F, F) {
    let t9832 = t9793 + t9794 + t9798 + t9799 + t9802 + t9810 + t9818 + t9829;
    let t9833 = t354 * t9832;
    let t9858 = t2999 * t889;
    (t9832, t9833, t9858)
}
