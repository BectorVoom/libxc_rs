//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1161/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1161<F: Float>(t28719: F, t317: F, t2842: F, t7091: F, t25462: F, t29026: F, t25491: F, t6963: F, t1466: F, t29034: F, t681: F, t28967: F, t28954: F, t28941: F, t29415: F, t92: F) -> (F, F, F, F, F, F, F, F, F) {
    let t112384 = t28719 * t317;
    let t112390 = t7091 * t2842;
    let t112402 = t25462 * t29026 / 27.0;
    let t112439 = t6963 * t25491 / 9.0;
    let t112449 = 2.0 / 9.0 * t1466 * t681 * t29034;
    let t112452 = 2.0 / 9.0 * t1466 * t681 * t28967;
    let t112463 = t1466 * t681 * t28954 / 9.0;
    let t112465 = 2.0 / 27.0 * t25462 * t28941;
    let t112479 = t29415 * t92;
    (t112384, t112390, t112402, t112439, t112449, t112452, t112463, t112465, t112479)
}
