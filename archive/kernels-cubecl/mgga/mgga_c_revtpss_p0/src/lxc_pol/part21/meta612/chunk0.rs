//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2364/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2364<F: Float>(t10573: F, t2619: F, t2598: F, t9321: F, t760: F, t2523: F, t9387: F, t2495: F, t39875: F, t9367: F, t10565: F, t606: F, t706: F) -> (F, F, F, F, F, F, F) {
    let t40127 = t10573 * t2619;
    let t40129 = t9321 * t2598;
    let t40131 = F::cast_from(0.21053605041484726346e2_f64) * t760 * t40129;
    let t40132 = t2523 * t9387;
    let t40135 = t9367 * t39875 * t2495;
    let t40137 = F::cast_from(0.6233709278045326953e3_f64) * t760 * t40135;
    let t40139 = t706 * t10565 * t606;
    (t40127, t40129, t40131, t40132, t40135, t40137, t40139)
}
