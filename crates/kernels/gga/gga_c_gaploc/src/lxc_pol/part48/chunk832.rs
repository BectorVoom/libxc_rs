//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 832/861 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk832<F: Float>(t1457: F, t1572: F, t44470: F, t13421: F, t1641: F, t13363: F, t37777: F, t11359: F, t40342: F, t13296: F, t1564: F, t2492: F, t4752: F, t3377: F, t38181: F, t1339: F, t1445: F, t1537: F, t1562: F, t2859: F, t3358: F, t44404: F, t46257: F, t46261: F, t46264: F, t46267: F, t46271: F, t46275: F, t46283: F, t46287: F, t46289: F, t46291: F, t46294: F, t475: F, t590: F) -> (F,) {
    let t46297 = 0.71500979903700853338e0 * t1572 * t1457 * t44470;
    let t46299 = 0.92023022289409799224e1 * t1641 * t13421;
    let t46301 = 0.42900587942220512003e1 * t37777 * t13363;
    let t46303 = 0.42900587942220512003e1 * t11359 * t40342;
    let t46304 = t1564 * t13296;
    let t46311 = 0.28600391961480341335e1 * t11359 * t4752 * t2492;
    let t46316 = 0.10725146985555128001e1 * t38181 * t3377;
    let t46317 = -t46257 - t46261 + t46264 - t46267 + t46271 + t46275 - 0.51123901271894332902e1 * t1537 * t1339 * t44404 * t590 + t46283 - t46287 - t46289 - t46291 + t46294 + t46297 - t46299 + t46301 + t46303 - 0.69017266717057349418e1 * t1562 * t1445 * t46304 * t475 + t46311 - 0.14300195980740170668e1 * t2859 * t4752 * t3358 - t46316;
    (t46317,)
}
