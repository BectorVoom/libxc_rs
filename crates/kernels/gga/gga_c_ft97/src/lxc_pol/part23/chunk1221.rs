//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1221/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1221<F: Float>(t226: F, t27703: F, t27565: F, t109200: F, t17836: F, t6: F, t24389: F, t39: F, t18712: F, t2441: F, t6035: F, t4917: F, t709: F, t2446: F, t66397: F, t689: F) -> (F, F, F, F, F, F, F) {
    let t123124 = t27703 * t226;
    let t123125 = t123124 * t27565;
    let t123129 = t17836 * t109200 * t6;
    let t123133 = t17836 * t24389 * t39;
    let t123142 = t6035 * t2441 * t18712;
    let t123145 = t4917 * t709;
    let t123156 = t6035 * t2446 * t18712;
    let t123165 = t66397 * t689;
    (t123125, t123129, t123133, t123142, t123145, t123156, t123165)
}
