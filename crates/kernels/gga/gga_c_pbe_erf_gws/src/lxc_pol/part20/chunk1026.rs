//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1026/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1026<F: Float>(t14570: F, t2134: F, t14059: F, t14073: F, t14080: F, t14085: F, t14554: F, t14556: F, t14558: F, t14560: F, t14563: F, t14568: F, t14504: F, t14527: F, t14553: F, t898: F) -> (F, F) {
    let t14571 = t2134 * t14570;
    let t14574 = 7.0 / 288.0 * t14554 - t14556 / 384.0 + 7.0 / 576.0 * t14558 - t14560 / 192.0 + 7.0 / 576.0 * t14059 + 7.0 / 144.0 * t14563 + t14568 / 96.0 - t14571 / 96.0 + t14073 + 7.0 / 1152.0 * t14080 + t14085;
    let t14576 = t14504 + t14527 + t14553 + t14574;
    let t14577 = t898 * t14576;
    (t14576, t14577)
}
