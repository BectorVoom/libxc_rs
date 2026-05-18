//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 583/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk583<F: Float>(t10268: F, t6507: F, t1358: F, t2299: F, t3394: F, t488: F, t2339: F, t7888: F, t3366: F, t605: F, t2902: F, t921: F) -> (F, F, F, F, F, F) {
    let t10269 = t6507 * t10268;
    let t10271 = F::new(0.63233348079280332442e-2) * t1358 * t10269;
    let t10272 = t2299 * t3394;
    let t10273 = t10272 * t488;
    let t10275 = F::new(0.31616674039640166221e-2) * t1358 * t10273;
    let t10276 = t7888 * t2339;
    let t10278 = F::new(0.94850022118920498663e-2) * t1358 * t10276;
    let t10295 = t3366 * t605;
    let t10298 = t2902 * t921;
    (t10271, t10272, t10275, t10278, t10295, t10298)
}
