//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 902/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk902<F: Float>(t20149: F, t6489: F, t1580: F, t4397: F, t6473: F, t2306: F, t4346: F, t19966: F, t20002: F, t4350: F, t6587: F, t20891: F, t20893: F, t20895: F, t20897: F, t1610: F, t6602: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t21946 = t20149 * t6489;
    let t21947 = t1580 * t21946;
    let t21956 = 0.59969295720591057378e-2 * t4397 * t6473;
    let t21969 = t2306 * t4346;
    let t21988 = 0.61905925925925925925e-2 * t19966;
    let t22002 = 0.15476481481481481481e-2 * t20002;
    let t22009 = t6587 * t4350;
    let t22035 = 0.25794135802469135802e-2 * t20891;
    let t22036 = 0.30952962962962962962e-2 * t20893;
    let t22037 = 0.10317654320987654321e-2 * t20895;
    let t22038 = 0.15476481481481481481e-2 * t20897;
    let t22056 = t6602 * t1610;
    (t21947, t21956, t21969, t21988, t22002, t22009, t22035, t22036, t22037, t22038, t22056)
}
