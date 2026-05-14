//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1165/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1165<F: Float>(t100453: F, t1570: F, t3188: F, t93378: F, t965: F, t100459: F, t1557: F, t28: F, t4436: F, t89: F, t93392: F, t22873: F, t4495: F, t101638: F, t102181: F, t102193: F, t102202: F, t116569: F, t93459: F, t93475: F, t93776: F) -> (F, F, F, F, F) {
    let t116574 = t93378 * t100453 * t965 * t1570 * t3188;
    let t116579 = t93378 * t100459 * t965 * t1557 * t3188;
    let t116583 = t89 * t28 * t93392 * t4436;
    let t116587 = t89 * t28 * t22873 * t4495;
    let t116590 = t116569 / 3.0 + t116574 / 3.0 - t116579 / 9.0 - 6.0 * t116583 + 2.0 * t116587 + t102181 - 8.0 / 9.0 * t101638 + t102193 + t93776 - t93459 + t93475 + t102202;
    (t116574, t116579, t116583, t116587, t116590)
}
