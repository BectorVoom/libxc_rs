//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1040/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1040<F: Float>(t3128: F, t898: F, t4791: F, t4794: F, t4798: F, t4806: F, t4972: F, t4975: F, t4979: F, t4981: F, t4984: F, t4992: F, t765: F, t7865: F, t7870: F, t9906: F, t9907: F, t9908: F) -> (F, F) {
    let t10288 = t898 * t3128;
    let t10292 = -t4972 + t4975 - t9906 - t9907 + t4979 + t4981 + t4984 + 0.857292e-1 * t7865 + 0.2025780996e0 * t765 * t10288 - t4791 + t4794 + t4798 - t4806 + t4992 - t9908 - 0.2025780996e0 * t7870;
    (t10288, t10292)
}
