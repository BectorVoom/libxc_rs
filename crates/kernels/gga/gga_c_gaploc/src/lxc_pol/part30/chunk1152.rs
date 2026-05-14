//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1152/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1152<F: Float>(t1457: F, t1966: F, t1991: F, t22213: F, t28570: F, t313: F, t32387: F, t33282: F, t33284: F, t33285: F, t33292: F, t33297: F, t33299: F, t33300: F, t33304: F, t33311: F, t33313: F, t33315: F, t33317: F, t33319: F, t33321: F, t590: F, t5983: F) -> (F,) {
    let t33322 = t28570 - 0.71500979903700853338e0 * t5983 * t1457 * t32387 - t33282 - t33284 + 0.42900587942220512003e1 * t313 * t33285 * t22213 - t33292 - t33297 - t33299 + 0.2044956050875773316e1 * t1991 * t33300 * t590 - 0.51123901271894332902e1 * t1966 * t33304 * t590 + t33311 + t33313 + t33315 + t33317 + t33319 + t33321;
    (t33322,)
}
