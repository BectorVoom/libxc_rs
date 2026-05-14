//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1049/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1049<F: Float>(t31812: F, t31816: F, t31822: F, t31825: F, t31832: F, t36289: F, t36293: F, t36294: F, t36296: F, t36300: F, t36303: F, t36306: F, t36308: F, t36310: F, t36314: F, t36320: F, t36325: F, t36327: F) -> (F,) {
    let t36329 = -0.18868855373762491241e-2 * t36289 + t36293 - 0.13976929906490734252e-2 * t36294 + 0.34299214494455789578e-1 * t36296 + t36300 + t36303 - 0.40015750243531754508e-2 * t31812 + 0.20007875121765877254e-2 * t31816 - t36306 / 24.0 - t36308 / 48.0 - t36310 / 48.0 - 0.22921875e-1 * t36314 + 11.0 / 1152.0 * t31822 + 0.34299214494455789578e-2 * t31825 - 0.21437009059034868486e-3 * t36320 - 0.10718504529517434243e-2 * t31832 - 0.21437009059034868486e-2 * t36325 - 0.94344276868812456204e-2 * t36327;
    (t36329,)
}
