//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 859/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk859<F: Float>(t41809: F, t493: F, t1441: F, t590: F, t1339: F, t41838: F, t1537: F, t1457: F, t1572: F, t40320: F, t42001: F, t42005: F, t42008: F, t42009: F, t42015: F, t42018: F, t42022: F, t42026: F, t42029: F, t42030: F, t42032: F, t42034: F, t42038: F, t42042: F, t42047: F) -> F {
    let t42048 = t493 * t41809;
    let t42051 = F::new(0.1022478025437886658e1) * t1441 * t42048 * t590;
    let t42052 = t1339 * t41838;
    let t42054 = t1537 * t42052 * t590;
    let t42059 = F::new(0.25561950635947166451e1) * t1537 * t1339 * t41809 * t590;
    let t42060 = -F::new(0.59584149919750711116e-1) * t42001 + t42005 + t42008 + F::new(0.71500979903700853338e0) * t1572 * t1457 * t42009 - F::new(0.13803453343411469884e2) * t42015 - t42018 - t42022 + F::new(0.72851559312449424384e1) * t40320 - F::new(0.29792074959875355558e-1) * t42026 + t42029 + F::new(0.71500979903700853338e0) * t42030 + F::new(0.71500979903700853338e0) * t42032 + F::new(0.71500979903700853338e0) * t42034 + F::new(0.20449560508757733161e1) * t42038 + F::new(0.30674340763136599742e1) * t42042 + t42047 + t42051 - F::new(0.51123901271894332902e1) * t42054 - t42059;
    t42060
}
