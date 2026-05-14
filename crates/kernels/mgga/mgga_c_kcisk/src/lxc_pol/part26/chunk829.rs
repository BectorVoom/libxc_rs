//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 829/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk829<F: Float>(t213: F, t300: F, t425: F, t1350: F, t1387: F, t1365: F, t3812: F, t1376: F, t3114: F, t1384: F, t3119: F, t1399: F, t3123: F, t11525: F, t435: F, t437: F) -> (F, F, F, F, F, F, F, F) {
    let t13964 = t213 * t300;
    let t13966 = 0.14055920378328537299e-1 * t13964 * t425;
    let t13967 = t1387 * t1350;
    let t13969 = t3812 * t1365;
    let t13989 = t3114 * t1376;
    let t14027 = t3119 * t1384;
    let t14029 = t3123 * t1399;
    let t14056 = 0.77488888888888888888e-2 * t435 * t11525 * t437;
    (t13964, t13966, t13967, t13969, t13989, t14027, t14029, t14056)
}
