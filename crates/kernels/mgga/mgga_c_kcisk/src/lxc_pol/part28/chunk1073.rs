//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1073/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1073<F: Float>(t18222: F, t18226: F, t22338: F, t22340: F, t22342: F, t22347: F, t22351: F, t22353: F, t22355: F, t22358: F, t5348: F, t9163: F, t5439: F, t9234: F, t2028: F, t10474: F, t11983: F, t16613: F, t16640: F, t18227: F, t18232: F, t18244: F, t18779: F, t1994: F, t22363: F, t22371: F, t22955: F, t22960: F, t22963: F, t22966: F, t22968: F, t22973: F, t22977: F, t22980: F, t7553: F, t7648: F) -> (F, F, F, F) {
    let t24545 = 0.38691203703703703703e-2 * t22338 - 0.46429444444444444444e-2 * t22340 + 0.15476481481481481481e-2 * t22342 + 0.193e0 * t5348 * t9163 + t18222 + 0.11607361111111111111e-2 * t22347 - 0.18571777777777777777e-1 * t22351 - t18226 + 0.12897067901234567901e-2 * t22353 + 0.77382407407407407407e-3 * t22355 + 0.23214722222222222222e-2 * t22358;
    let t24561 = t9234 * t5439;
    let t24562 = t24561 * t2028;
    let t24569 = -t18227 + 0.38691203703703703703e-3 * t22363 + 0.148996e0 * t18779 * t7553 + 0.386e0 * t7648 * t7553 - 0.61905925925925925925e-2 * t16613 + t18232 - 0.23214722222222222222e-2 * t22371 - 0.17411041666666666666e-2 * t22955 + 0.46429444444444444444e-2 * t22960 + 0.12381185185185185185e-1 * t22963 - 0.10317654320987654321e-1 * t22966 + 0.15476481481481481481e-2 * t22968 - 0.30952962962962962962e-2 * t22973 + 0.193e0 * t1994 * t24562 - 0.25794135802469135802e-3 * t10474 - 0.23214722222222222221e-2 * t22977 - 0.61905925925925925924e-2 * t22980 + t11983 - 0.77382407407407407407e-3 * t16640 - t18244;
    (t24545, t24561, t24562, t24569)
}
