//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 743/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk743<F: Float>(t2558: F, t28438: F, t10928: F, t6574: F, t822: F, t2012: F, t7809: F, t7802: F, t5638: F, t9419: F, t5538: F, t883: F) -> (F, F, F, F, F, F) {
    let t28439 = t28438 * t2558;
    let t28640 = t822 * t10928 * t6574;
    let t28673 = t2012 * t7809;
    let t28737 = t2012 * t7802;
    let t28856 = t822 * t5638 * t9419;
    let t29277 = t5538 * t883;
    (t28439, t28640, t28673, t28737, t28856, t29277)
}
