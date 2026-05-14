//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 490/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk490<F: Float>(t10268: F, t6507: F, t1358: F, t2299: F, t3394: F, t488: F, t2339: F, t7888: F, t3366: F, t605: F, t2902: F, t921: F, t1016: F, t2497: F, t3381: F, t4379: F) -> (F, F, F, F, F, F, F, F) {
    let t10269 = t6507 * t10268;
    let t10271 = 0.63233348079280332442e-2 * t1358 * t10269;
    let t10272 = t2299 * t3394;
    let t10273 = t10272 * t488;
    let t10275 = 0.31616674039640166221e-2 * t1358 * t10273;
    let t10276 = t7888 * t2339;
    let t10278 = 0.94850022118920498663e-2 * t1358 * t10276;
    let t10295 = t3366 * t605;
    let t10298 = t2902 * t921;
    let t10301 = t1016 * t2497;
    let t10308 = t4379 * t3381;
    (t10271, t10272, t10275, t10278, t10295, t10298, t10301, t10308)
}
