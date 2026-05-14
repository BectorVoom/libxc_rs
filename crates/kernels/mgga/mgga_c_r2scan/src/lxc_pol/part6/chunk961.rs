//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 961/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk961<F: Float>(t4791: F, t4794: F, t4798: F, t4806: F, t4972: F, t4975: F, t4979: F, t4981: F, t4992: F, t4996: F, t5000: F, t5004: F, t6960: F, t6961: F, t6975: F, t7009: F) -> (F,) {
    let t7152 = t4972 - t4975 + t6960 - t4979 - t4981 + t6961 + t4791 - t4794 - t4798 + t4806 - t6975 - t4992 + t7009 + t4996 - t5000 - t5004;
    (t7152,)
}
