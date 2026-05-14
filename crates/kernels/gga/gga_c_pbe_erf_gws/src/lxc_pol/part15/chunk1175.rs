//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1175/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1175<F: Float>(t13893: F, t4150: F, t4002: F, t8669: F, t8743: F, t13808: F, t14596: F, t3965: F, t9299: F, t52036: F, t1115: F, t50932: F, t52020: F, t52027: F, t54707: F, t54711: F, t54714: F, t54717: F, t54719: F, t54722: F, t827: F) -> (F,) {
    let t54724 = t13893 * t4150;
    let t54727 = 7.0 / 144.0 * t8669 * t4002;
    let t54729 = 7.0 / 144.0 * t8743 * t4002;
    let t54730 = t13808 * t14596;
    let t54731 = 7.0 / 1152.0 * t54730;
    let t54734 = t3965 * t9299;
    let t54737 = 35.0 / 216.0 * t52036;
    let t54738 = -t54707 / 768.0 - t827 * t54711 / 48.0 + t54714 / 24.0 + t54717 - 35.0 / 216.0 * t52020 - 35.0 / 216.0 * t54719 - t54722 / 48.0 - 119.0 / 13824.0 * t54724 + t54727 + t54729 + t54731 - t1115 * t50932 / 96.0 - t54734 / 16.0 + 7.0 / 36.0 * t52027 + t54737;
    (t54738,)
}
