//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1169/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1169<F: Float>(t26179: F, t29548: F, t29554: F, t7349: F, t28640: F, t7709: F, t29562: F, t95319: F, t108978: F, t2047: F, t108986: F, t116: F, t30552: F) -> (F, F, F, F, F, F, F) {
    let t110016 = t26179 * t29548;
    let t110018 = t29554 * t7349;
    let t110020 = t7709 * t28640;
    let t110022 = t95319 * t29562;
    let t110039 = t2047 * t108978;
    let t110044 = t2047 * t108986;
    let t110110 = t30552 * t116;
    (t110016, t110018, t110020, t110022, t110039, t110044, t110110)
}
