//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 977/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk977<F: Float>(t37326: F, t895: F, t1000: F, t10215: F, t10497: F, t13253: F, t13360: F, t1445: F, t1562: F, t1641: F, t2819: F, t2823: F, t2854: F, t34131: F, t4130: F, t42026: F, t44404: F, t44564: F, t4614: F, t46471: F, t46473: F, t46480: F, t46490: F, t46491: F, t46497: F, t46498: F, t46501: F, t46504: F, t46507: F, t4781: F, t590: F, t597: F) -> F {
    let t46516 = F::new(0.23833659967900284446e0) * t895 * t37326;
    let t46517 = F::new(0.23005755572352449806e2) * t597 * t1445 * t44564 + t46471 + t46473 + F::new(0.71500979903700853338e0) * t1000 * t34131 - F::new(0.92023022289409799224e1) * t1641 * t13360 + F::new(0.12780975317973583226e0) * t46480 - F::new(0.59584149919750711116e-1) * t42026 + F::new(0.30674340763136599742e1) * t4781 * t4130 * t44404 * t590 - t46490 + t46491 + F::new(0.71500979903700853338e0) * t2823 * t10497 + F::new(0.71500979903700853338e0) * t2819 * t10497 + t46497 + t46498 + t46501 - t46504 - t46507 - F::new(0.13803453343411469884e2) * t1562 * t1445 * t2854 * t10215 + F::new(0.30674340763136599741e2) * t597 * t4614 * t13253 + t46516;
    t46517
}
