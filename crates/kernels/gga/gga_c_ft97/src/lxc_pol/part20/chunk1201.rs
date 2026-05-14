//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1201/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1201<F: Float>(t15133: F, t6386: F, t10688: F, t28847: F, t25491: F, t6963: F, t25183: F, t4246: F, t7021: F, t7640: F, t1466: F, t29034: F, t681: F, t28967: F, t10409: F, t193: F, t2405: F, t25391: F, t28985: F, t29017: F, t6210: F, t6216: F, t98333: F, t98341: F, t98342: F, t98351: F, t98354: F) -> (F, F, F, F, F) {
    let t112426 = t15133 * t6386;
    let t112432 = t10688 * t28847;
    let t112439 = t6963 * t25491 / 9.0;
    let t112441 = t4246 * t25183;
    let t112443 = t7640 * t7021;
    let t112449 = 2.0 / 9.0 * t1466 * t681 * t29034;
    let t112452 = 2.0 / 9.0 * t1466 * t681 * t28967;
    let t112455 = -4.0 * t112426 - t6216 * t10409 * t28985 * t2405 / 27.0 + 8.0 * t112432 + t98333 / 27.0 - t98341 / 3.0 + t6210 * t29017 / 3.0 - t112439 - t98342 / 27.0 - 2.0 * t112441 + t1466 * t193 * t112443 * t25391 + t112449 + t112452 + 2.0 / 9.0 * t98351 + t98354 / 9.0;
    (t112426, t112432, t112441, t112443, t112455)
}
