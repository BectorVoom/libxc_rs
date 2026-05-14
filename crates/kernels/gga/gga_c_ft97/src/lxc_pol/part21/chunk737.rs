//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 737/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk737<F: Float>(t15742: F, t3613: F, t12137: F, t15737: F, t15746: F, t2266: F, t3653: F, t925: F, t2253: F, t4874: F, t4885: F, t1073: F, t920: F, t363: F, t12122: F, t358: F, t8680: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t17613 = t3613 * t15742;
    let t17616 = t12137 * t15737;
    let t17619 = t3613 * t15746;
    let t17623 = t2266 * t925 * t3653;
    let t17626 = t2253 * t4874;
    let t17627 = t2253 * t4885;
    let t17630 = t920 * t1073;
    let t17631 = t17630 * t363;
    let t17632 = t12122 * t17631;
    let t17636 = t8680 * t358;
    (t17613, t17616, t17619, t17623, t17626, t17627, t17630, t17631, t17632, t17636)
}
