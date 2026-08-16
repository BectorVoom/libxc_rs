//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1187/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1187(t12019: f64, t566: f64, t68: f64, t3888: f64, t12023: f64, t12027: f64, t12030: f64, t12033: f64, t12181: f64, t12237: f64, t12238: f64, t12240: f64, t12249: f64, t12251: f64, t12252: f64, t12259: f64, t12267: f64, t12434: f64, t12438: f64, t1323: f64, t1336: f64, t1352: f64, t1372: f64, t1375: f64, t1378: f64, t1380: f64, t1381: f64, t1383: f64, t22694: f64, t22740: f64, t3752: f64, t3758: f64, t3777: f64, t3793: f64, t3851: f64, t3879: f64, t3882: f64, t3889: f64, t3897: f64, t3898: f64, t3902: f64, t3907: f64, t39938: f64, t40047: f64, t40118: f64, t40133: f64, t40148: f64, t40153: f64, t40162: f64, t40438: f64, t40453: f64, t40475: f64, t40479: f64, t40486: f64, t40492: f64, t40524: f64, t40541: f64, t40576: f64, t5250: f64, t5334: f64, t5344: f64, t539: f64, t562: f64, t568: f64) -> f64 {
    let t40590 = 1.0_f64 / t12019 / t566;
    let t40591 = t68 * t40590;
    let t40592 = t3888 * t3888;
    let t40603 = t539 * t40453 * t568 + 12.0_f64 * t12030 * t3889 + 12.0_f64 * t12033 * t3889 - 24.0_f64 * t3758 * t12023 + 24.0_f64 * t3882 * t12027 - t1375 * t1378 * (-24.0_f64 * t1336 * t40492 * t12251 - 6.0_f64 * t1336 * t12259 * t3851 - 4.0_f64 * t1336 * t40479 * t1352 - 3.0_f64 * t1336 * t1380 * t39938 + 12.0_f64 * t1336 * t40486 * t3793 + 6.0_f64 * t1336 * t3897 * t40133 - 6.0_f64 * t5344 * t22740 * t3851 + 8.0_f64 * t5334 * t40475 * t5250 - 24.0_f64 * t3777 * t12252 - 12.0_f64 * t12267 * t3902 + t40524 + 24.0_f64 * t5334 * t22694 * t12240 - 36.0_f64 * t1336 * t12249 * t40148 - t1336 * t1380 * t40153 + 14.0_f64 * t1336 * t3897 * t40162 + 24.0_f64 * t1336 * t40541 * t40047 - 12.0_f64 * t3777 * t12181 + 4.0_f64 * t12238 * t1383 + 12.0_f64 * t12267 * t3898 - 6.0_f64 * t12267 * t3907 - 4.0_f64 * t40118 * t1381 + t40576) + t40438 * t562 * t568 + 4.0_f64 * t12237 * t1372 * t568 + 6.0_f64 * t3752 * t3879 * t568 + 24.0_f64 * t1375 * t40591 * t40592 - 24.0_f64 * t3882 * t12023 - 4.0_f64 * t3882 * t12438 + 4.0_f64 * t1323 * t12434 * t568;
    t40603
}
