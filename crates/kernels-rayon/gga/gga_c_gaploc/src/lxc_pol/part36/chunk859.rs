//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 859/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk859(t41809: f64, t493: f64, t1441: f64, t590: f64, t1339: f64, t41838: f64, t1537: f64, t1457: f64, t1572: f64, t40320: f64, t42001: f64, t42005: f64, t42008: f64, t42009: f64, t42015: f64, t42018: f64, t42022: f64, t42026: f64, t42029: f64, t42030: f64, t42032: f64, t42034: f64, t42038: f64, t42042: f64, t42047: f64) -> f64 {
    let t42048 = t493 * t41809;
    let t42051 = 0.1022478025437886658e1_f64 * t1441 * t42048 * t590;
    let t42052 = t1339 * t41838;
    let t42054 = t1537 * t42052 * t590;
    let t42059 = 0.25561950635947166451e1_f64 * t1537 * t1339 * t41809 * t590;
    let t42060 = -0.59584149919750711116e-1_f64 * t42001 + t42005 + t42008 + 0.71500979903700853338e0_f64 * t1572 * t1457 * t42009 - 0.13803453343411469884e2_f64 * t42015 - t42018 - t42022 + 0.72851559312449424384e1_f64 * t40320 - 0.29792074959875355558e-1_f64 * t42026 + t42029 + 0.71500979903700853338e0_f64 * t42030 + 0.71500979903700853338e0_f64 * t42032 + 0.71500979903700853338e0_f64 * t42034 + 0.20449560508757733161e1_f64 * t42038 + 0.30674340763136599742e1_f64 * t42042 + t42047 + t42051 - 0.51123901271894332902e1_f64 * t42054 - t42059;
    t42060
}
