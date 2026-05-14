//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1374/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1374<F: Float>(t26088: F, t6537: F, t6403: F, t7601: F, t1234: F, t1570: F, t1592: F, t19807: F, t20499: F, t20721: F, t2122: F, t2124: F, t2139: F, t22749: F, t24288: F, t25263: F, t26053: F, t26060: F, t26062: F, t26066: F, t26085: F, t2651: F, t2654: F, t2719: F, t2841: F, t5109: F, t5137: F, t551: F, t552: F, t6334: F, t6389: F, t7337: F, t910: F) -> (F,) {
    let t26089 = t26088 * t6537;
    let t26091 = t7601 * t6403;
    let t26093 = 0.7801399566048841707e0 * t20721 * t551 * t552 * t910 * t5137 + t26053 + 0.39006997830244208535e0 * t1592 * t551 * t552 * t2719 * t1234 + 0.26023093918533882312e-2 * t26060 + 0.19207560116895242163e0 * t26062 - 0.43341108700271342816e-1 * t2651 * t6389 - 0.98781737744032673976e0 * t2122 * t7337 * t26066 + 0.31205598264195366828e1 * t20499 * t5109 * t25263 + 0.7801399566048841707e0 * t19807 * t5109 * t2654 * t1570 + 0.39006997830244208535e0 * t2139 * t5109 * t24288 - 0.19756347548806534797e1 * t22749 * t2124 * t2841 * t6334 - 0.16463622957338778996e-1 * t26085 + 0.34930954652346593433e-1 * t26089 - 0.17465477326173296717e-1 * t26091;
    (t26093,)
}
