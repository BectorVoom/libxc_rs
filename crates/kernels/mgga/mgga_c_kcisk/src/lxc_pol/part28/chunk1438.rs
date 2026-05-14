//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1438/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1438<F: Float>(t113181: F, t118120: F, t118180: F, t118334: F, t121800: F, t121803: F, t121806: F, t121809: F, t121812: F, t121815: F, t121818: F, t121828: F, t122899: F, t24992: F, t24997: F, t34548: F, t35410: F, t9728: F, t9748: F) -> (F,) {
    let t123140 = -0.69444444444444444444e-2 * t113181 * t118334 * t24992 - 0.34722222222222222222e-2 * t113181 * t122899 - 0.69444444444444444444e-2 * t113181 * t118180 * t24997 - 0.3574074074074074074e-2 * t118120 * t34548 - 0.27777777777777777778e-1 * t35410 * t9748 + 0.12381185185185185185e-1 * t121800 - 0.10317654320987654321e-1 * t121803 + 0.15476481481481481481e-2 * t121806 - 0.30952962962962962962e-2 * t121809 + 0.23214722222222222222e-2 * t121812 - 0.15476481481481481481e-2 * t121815 + 0.69644166666666666666e-2 * t121818 - 0.27777777777777777778e-1 * t35410 * t9728 + 0.12897067901234567901e-2 * t121828;
    (t123140,)
}
