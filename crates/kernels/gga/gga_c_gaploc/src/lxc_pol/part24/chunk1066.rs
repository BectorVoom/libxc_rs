//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1066/1270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1066<F: Float>(t10156: F, t10157: F, t1064: F, t1266: F, t2268: F, t31766: F, t31772: F, t31777: F, t31783: F, t31786: F, t31788: F, t31790: F, t31792: F, t31796: F, t31799: F, t31800: F, t31805: F, t31811: F, t6305: F) -> (F,) {
    let t31814 = t31766 - t31772 + t31777 - t31783 + t31786 - t31788 - t31790 - t31792 - t31796 + t31799 - 0.1707300398140568976e0 * t2268 * t1064 * t31800 + t31805 - 0.85365019907028448797e-1 * t2268 * t10156 * t1266 + t31811 - 0.1707300398140568976e0 * t6305 * t10157;
    (t31814,)
}
