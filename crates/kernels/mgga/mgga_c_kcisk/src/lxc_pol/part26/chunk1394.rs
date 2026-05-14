//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1394/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1394<F: Float>(t109633: F, t109664: F, t115118: F, t115806: F, t119494: F, t119497: F, t119501: F, t119513: F, t119534: F, t120036: F, t120448: F, t32339: F, t32436: F, t32439: F, t34936: F, t34955: F, t35008: F, t9512: F, t9524: F, t9864: F) -> (F,) {
    let t120670 = 0.46296296296296296296e-2 * t32339 * t35008 + 0.40208333333333333335e-2 * t32439 * t120036 + 0.19345601851851851852e-2 * t119494 - 0.10416666666666666667e-1 * t9512 * t34936 + 0.15476481481481481481e-2 * t119497 - 0.11607361111111111111e-2 * t119501 - 0.10416666666666666667e-1 * t9524 * t34936 + 0.23214722222222222221e-2 * t119513 - 0.34722222222222222222e-2 * t115118 * t9864 - 0.13402777777777777778e-2 * t109633 * t120448 + 0.34722222222222222222e-2 * t32436 * t34955 + 0.13402777777777777778e-2 * t109664 * t34955 - 0.46429444444444444444e-2 * t119534 - 0.92592592592592592592e-2 * t115806;
    (t120670,)
}
