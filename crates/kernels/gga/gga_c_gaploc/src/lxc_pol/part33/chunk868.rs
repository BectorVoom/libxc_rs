//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 868/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk868<F: Float>(t2312: F, t3344: F, t2321: F, t2822: F, t882: F, t2765: F, t6750: F, t1063: F, t10241: F, t6508: F, t6507: F, t1358: F, t2299: F, t3394: F, t488: F, t2339: F, t7888: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t10260 = t2312 * t3344;
    let t10261 = 0.11856252764865062333e-2 * t10260;
    let t10262 = t2822 * t2321;
    let t10263 = t882 * t10262;
    let t10264 = 0.11856252764865062333e-2 * t10263;
    let t10265 = t2765 * t6750;
    let t10267 = 0.85365019907028448797e-1 * t1063 * t10265;
    let t10268 = t6508 * t10241;
    let t10269 = t6507 * t10268;
    let t10271 = 0.63233348079280332442e-2 * t1358 * t10269;
    let t10272 = t2299 * t3394;
    let t10273 = t10272 * t488;
    let t10275 = 0.31616674039640166221e-2 * t1358 * t10273;
    let t10276 = t7888 * t2339;
    (t10261, t10262, t10264, t10265, t10267, t10268, t10269, t10271, t10273, t10275, t10276)
}
