//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 744/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk744<F: Float>(t2547: F, t279: F, t481: F, t122: F, t2310: F, t4260: F, t883: F, t2321: F, t28438: F, t4389: F, t899: F, t1415: F) -> (F, F, F, F, F, F) {
    let t29439 = t481 * t2547 * t279;
    let t29874 = t481 * t2310 * t122;
    let t30204 = t4260 * t883;
    let t30733 = t28438 * t2321;
    let t30829 = t4389 * t899;
    let t30830 = t1415 * t30829;
    (t29439, t29874, t30204, t30733, t30829, t30830)
}
