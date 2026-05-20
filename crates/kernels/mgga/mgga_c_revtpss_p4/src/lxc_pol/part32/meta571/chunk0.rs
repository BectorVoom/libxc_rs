//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1895/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1895<F: Float>(t101218: F, t2047: F, t28154: F, t95296: F, t28147: F, t95319: F, t28150: F, t7348: F, t25162: F, t116: F, t28651: F, t2106: F, t47672: F) -> (F, F, F, F, F, F, F) {
    let t101938 = t2047 * t101218;
    let t101955 = F::new(160.0) / F::new(9.0) * t28154 * t95296;
    let t101969 = F::new(160.0) / F::new(3.0) * t95319 * t28147;
    let t101970 = t7348 * t28150;
    let t101972 = F::new(160.0) / F::new(9.0) * t25162 * t101970;
    let t102019 = t28651 * t116;
    let t102070 = t2106 * t47672;
    (t101938, t101955, t101969, t101970, t101972, t102019, t102070)
}
