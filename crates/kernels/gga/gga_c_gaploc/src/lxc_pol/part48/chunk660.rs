//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 660/861 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk660<F: Float>(t107: F, t408: F, t2558: F, t10928: F, t6574: F, t822: F, t2012: F, t7809: F, t7802: F, t5638: F, t9419: F, t5538: F, t883: F, t2547: F, t279: F, t481: F) -> (F, F, F, F, F, F, F, F) {
    let t28438 = t107 * t408;
    let t28439 = t28438 * t2558;
    let t28640 = t822 * t10928 * t6574;
    let t28673 = t2012 * t7809;
    let t28737 = t2012 * t7802;
    let t28856 = t822 * t5638 * t9419;
    let t29277 = t5538 * t883;
    let t29439 = t481 * t2547 * t279;
    (t28438, t28439, t28640, t28673, t28737, t28856, t29277, t29439)
}
