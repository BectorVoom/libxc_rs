//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1127/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1127<F: Float>(t19694: F, t3200: F, t95848: F, t19698: F, t92808: F, t19750: F, t95911: F, t1020: F, t4792: F, t95664: F, t19149: F, t4994: F, t7718: F, t1252: F, t15239: F, t18681: F, t26960: F, t26961: F, t27014: F, t29112: F, t3515: F, t7788: F, t7789: F, t84759: F, t84812: F, t93145: F, t97360: F) -> (F, F, F, F, F, F) {
    let t100762 = t3200 * t95848 * t19694;
    let t100765 = t3200 * t92808 * t19698;
    let t100768 = t3200 * t95911 * t19750;
    let t100778 = t1020 * t95664 * t4792;
    let t100781 = t4994 * t7718 * t19149;
    let t100783 = -t97360 + 0.11584201388888888889e-3 * t26960 * t3515 * t26961 * t84759 + 0.46336805555555555556e-3 * t26960 * t15239 * t26961 * t84812 + 0.46429444444444444444e-2 * t100762 - 0.15476481481481481481e-2 * t100765 + 0.46429444444444444444e-2 * t100768 - 0.38691203703703703703e-3 * t93145 - 0.11584201388888888889e-3 * t7788 * t1252 * t7789 * t18681 - 0.15445601851851851852e-3 * t27014 * t29112 - 0.23214722222222222221e-2 * t100778 + 0.30952962962962962962e-2 * t100781;
    (t100762, t100765, t100768, t100778, t100781, t100783)
}
