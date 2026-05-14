//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1151/1270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1151<F: Float>(t20671: F, t25070: F, t28856: F, t10668: F, t11006: F, t1457: F, t1991: F, t2052: F, t28564: F, t28567: F, t32234: F, t33246: F, t33248: F, t33253: F, t33255: F, t33257: F, t33259: F, t33261: F, t33269: F, t33271: F, t3464: F, t5782: F, t590: F, t6060: F, t6096: F) -> (F,) {
    let t33273 = t28856 * t20671 * t25070;
    let t33274 = 0.2556195063594716645e0 * t33273;
    let t33275 = 0.1022478025437886658e1 * t1991 * t10668 * t590 + t33246 + t33248 - 0.21450293971110256001e1 * t6060 * t1457 * t32234 - t33253 - t33255 - t33257 - t33259 + t33261 + 0.71500979903700853338e0 * t2052 * t3464 * t6096 - 0.13803453343411469884e2 * t5782 * t11006 + t33269 + t33271 + t33274 + t28564 + t28567;
    (t33275,)
}
