//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 977/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk977(t37326: f64, t895: f64, t1000: f64, t10215: f64, t10497: f64, t13253: f64, t13360: f64, t1445: f64, t1562: f64, t1641: f64, t2819: f64, t2823: f64, t2854: f64, t34131: f64, t4130: f64, t42026: f64, t44404: f64, t44564: f64, t4614: f64, t46471: f64, t46473: f64, t46480: f64, t46490: f64, t46491: f64, t46497: f64, t46498: f64, t46501: f64, t46504: f64, t46507: f64, t4781: f64, t590: f64, t597: f64) -> f64 {
    let t46516 = 0.23833659967900284446e0_f64 * t895 * t37326;
    let t46517 = 0.23005755572352449806e2_f64 * t597 * t1445 * t44564 + t46471 + t46473 + 0.71500979903700853338e0_f64 * t1000 * t34131 - 0.92023022289409799224e1_f64 * t1641 * t13360 + 0.12780975317973583226e0_f64 * t46480 - 0.59584149919750711116e-1_f64 * t42026 + 0.30674340763136599742e1_f64 * t4781 * t4130 * t44404 * t590 - t46490 + t46491 + 0.71500979903700853338e0_f64 * t2823 * t10497 + 0.71500979903700853338e0_f64 * t2819 * t10497 + t46497 + t46498 + t46501 - t46504 - t46507 - 0.13803453343411469884e2_f64 * t1562 * t1445 * t2854 * t10215 + 0.30674340763136599741e2_f64 * t597 * t4614 * t13253 + t46516;
    t46517
}
