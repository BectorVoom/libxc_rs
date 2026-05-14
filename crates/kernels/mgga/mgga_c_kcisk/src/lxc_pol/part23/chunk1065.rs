//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1065/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1065<F: Float>(t1459: F, t21341: F, t2339: F, t4534: F, t2347: F, t4536: F, t12836: F, t12838: F, t12842: F, t18946: F, t18951: F, t18956: F, t18960: F, t18965: F, t18969: F, t18973: F, t18976: F, t18979: F, t18982: F, t18987: F, t18991: F, t18995: F, t19000: F, t19008: F, t19011: F, t19014: F) -> (F, F, F, F) {
    let t21342 = t1459 * t21341;
    let t21345 = t2339 * t4534;
    let t21348 = t2347 * t4536;
    let t21371 = -0.10317654320987654321e-1 * t18946 + 0.77382407407407407407e-3 * t18951 - 0.23214722222222222222e-2 * t18956 + 0.11349419753086419753e-1 * t18960 + 0.38691203703703703704e-2 * t18965 - 0.11607361111111111111e-2 * t18969 - 0.19345601851851851852e-2 * t18973 - 0.23214722222222222222e-2 * t18976 - 0.41270617283950617284e-2 * t18979 + 0.12381185185185185185e-1 * t18982 - 0.51588271604938271604e-3 * t18987 - 0.15476481481481481481e-2 * t18991 - 0.15476481481481481481e-2 * t18995 + 0.46429444444444444443e-2 * t19000 + 0.15476481481481481481e-2 * t12836 + 0.77382407407407407407e-3 * t12838 + 0.12897067901234567901e-2 * t12842 - 0.23214722222222222222e-2 * t19008 + 0.69644166666666666664e-2 * t19011 - 0.23214722222222222222e-2 * t19014;
    (t21342, t21345, t21348, t21371)
}
