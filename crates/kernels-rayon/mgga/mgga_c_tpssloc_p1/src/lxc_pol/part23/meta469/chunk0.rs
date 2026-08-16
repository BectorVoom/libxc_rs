//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1386/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1386(t1021: f64, t10403: f64, t1041: f64, t10480: f64, t10883: f64, t10970: f64, t1409: f64, t17712: f64, t21405: f64, t21532: f64, t248: f64, t3071: f64, t3146: f64, t360: f64, t42358: f64, t4582: f64, t48670: f64, t48674: f64, t49934: f64, t50193: f64, t5878: f64, t61782: f64, t62079: f64, t62840: f64, t70100: f64, t70239: f64, t70346: f64, t70351: f64, t70363: f64, t70389: f64, t70404: f64, t75847: f64, t76581: f64, t76740: f64, t973: f64, t974: f64) -> f64 {
    let t77539 = t50193 * t21405 / 768.0_f64 - t42358 * t248 * t1021 * t76740 * t360 / 3072.0_f64 - 5.0_f64 / 432.0_f64 * t1041 * t248 * t10970 * t76581 - 5.0_f64 / 864.0_f64 * t70239 + t10403 * t3071 * t62840 * t70100 * t1409 / 192.0_f64 - t61782 / 3456.0_f64 + t973 * t974 * t3146 * t75847 / 72.0_f64 + t10883 * t4582 * t17712 * t5878 / 512.0_f64 - t49934 * t21532 / 384.0_f64 + t70346 / 1152.0_f64 - t70351 / 384.0_f64 + t70363 / 1152.0_f64 + t48670 / 2592.0_f64 + t48674 / 3888.0_f64 + 5.0_f64 / 1944.0_f64 * t70389 + 3.0_f64 / 256.0_f64 * t10480 * t4582 * t17712 * t62079 - t70404 / 288.0_f64;
    t77539
}
