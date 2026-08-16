//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3216/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3216(t113: f64, t12545: f64, t1271: f64, t12816: f64, t1393: f64, t1458: f64, t15857: f64, t16503: f64, t1778: f64, t1849: f64, t19289: f64, t19537: f64, t20098: f64, t20136: f64, t2312: f64, t2314: f64, t3652: f64, t3660: f64, t3929: f64, t4028: f64, t4034: f64, t510: f64, t513: f64, t5450: f64, t55568: f64, t55927: f64, t56110: f64, t56124: f64, t56148: f64, t56161: f64, t56174: f64, t56192: f64, t56212: f64, t56294: f64, t56364: f64, t56370: f64, t56389: f64, t56408: f64, t57801: f64, t57810: f64, t57815: f64, t57822: f64, t6287: f64, t6295: f64, t63261: f64, t6468: f64, t650: f64, t652: f64, t66921: f64) -> f64 {
    let t66935 = -8.0_f64 * t4034 * t20136 - 4.0_f64 * t652 * t15857 * t1458 - 8.0_f64 * t2314 * t20136 - 8.0_f64 * t4028 * t12545 - 2.0_f64 * t652 * t510 * t55568 + 2.0_f64 * t12816 * t1849 + 2.0_f64 * t19537 * t1393 + t513 * (t56110 + t56124 + t56148 + t56161 + t56174 + t56192 + t56212 + t56294 + t56364 + t56370 + t56389 + t56408 + t57801 + t57810 + t57815 + t57822) - t113 * (t63261 + t66921) - t2312 * t6287 - 2.0_f64 * t650 * t19289 - t55927 * t510 - t5450 * t3652 + t6295 * t3929 + 2.0_f64 * t1271 * t20098 + t3660 * t6468 + 2.0_f64 * t1778 * t16503;
    t66935
}
