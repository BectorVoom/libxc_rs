//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1087/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1087<F: Float>(t5281: F, t5341: F, t1240: F, t13303: F, t13312: F, t15179: F, t15182: F, t15189: F, t15191: F, t15192: F, t19661: F, t19664: F, t19682: F, t19686: F, t19689: F, t19692: F, t19696: F, t19700: F, t3638: F, t6843: F, t9552: F, t9563: F) -> (F, F) {
    let t20294 = t5281 * t5341;
    let t20309 = -0.34822083333333333332e-2 * t19661 + t15179 + 0.69644166666666666664e-2 * t19664 + 0.13345e0 * t1240 * t20294 - 0.25794135802469135802e-3 * t9552 - t15182 + t15189 + 0.77382407407407407407e-3 * t13303 + t15191 + t15192 - 0.41270617283950617283e-2 * t13312 - 0.25794135802469135802e-3 * t9563 - 0.38691203703703703703e-2 * t19682 - 0.30952962962962962963e-2 * t19686 + 0.46429444444444444444e-2 * t19689 + 0.23214722222222222222e-2 * t19692 + 0.46429444444444444444e-2 * t19696 - 0.15476481481481481481e-2 * t19700 - 0.66725e-1 * t3638 * t6843;
    (t20294, t20309)
}
