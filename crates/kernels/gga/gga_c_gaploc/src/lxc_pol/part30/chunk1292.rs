//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1292/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1292<F: Float>(t10924: F, t5679: F, t6096: F, t11069: F, t5669: F, t20671: F, t25070: F, t28856: F, t10668: F, t11006: F, t1457: F, t1991: F, t2052: F, t28564: F, t28567: F, t32234: F, t33246: F, t33248: F, t33253: F, t33255: F, t33257: F, t33259: F, t33261: F, t3464: F, t5782: F, t590: F, t6060: F) -> F {
    let t33269 = F::cast_from(0.71500979903700853338e0_f64) * t5679 * t10924 * t6096;
    let t33271 = F::cast_from(0.2044956050875773316e1_f64) * t5669 * t11069;
    let t33273 = t28856 * t20671 * t25070;
    let t33274 = F::cast_from(0.2556195063594716645e0_f64) * t33273;
    let t33275 = F::cast_from(0.1022478025437886658e1_f64) * t1991 * t10668 * t590 + t33246 + t33248 - F::cast_from(0.21450293971110256001e1_f64) * t6060 * t1457 * t32234 - t33253 - t33255 - t33257 - t33259 + t33261 + F::cast_from(0.71500979903700853338e0_f64) * t2052 * t3464 * t6096 - F::cast_from(0.13803453343411469884e2_f64) * t5782 * t11006 + t33269 + t33271 + t33274 + t28564 + t28567;
    t33275
}
