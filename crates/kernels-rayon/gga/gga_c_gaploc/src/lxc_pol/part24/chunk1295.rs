//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1295/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1295(t10924: f64, t5679: f64, t6096: f64, t11069: f64, t5669: f64, t20671: f64, t25070: f64, t28856: f64, t10668: f64, t11006: f64, t1457: f64, t1991: f64, t2052: f64, t28564: f64, t28567: f64, t32234: f64, t33246: f64, t33248: f64, t33253: f64, t33255: f64, t33257: f64, t33259: f64, t33261: f64, t3464: f64, t5782: f64, t590: f64, t6060: f64) -> f64 {
    let t33269 = 0.71500979903700853338e0_f64 * t5679 * t10924 * t6096;
    let t33271 = 0.2044956050875773316e1_f64 * t5669 * t11069;
    let t33273 = t28856 * t20671 * t25070;
    let t33274 = 0.2556195063594716645e0_f64 * t33273;
    let t33275 = 0.1022478025437886658e1_f64 * t1991 * t10668 * t590 + t33246 + t33248 - 0.21450293971110256001e1_f64 * t6060 * t1457 * t32234 - t33253 - t33255 - t33257 - t33259 + t33261 + 0.71500979903700853338e0_f64 * t2052 * t3464 * t6096 - 0.13803453343411469884e2_f64 * t5782 * t11006 + t33269 + t33271 + t33274 + t28564 + t28567;
    t33275
}
