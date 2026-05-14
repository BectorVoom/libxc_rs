//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1034/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1034<F: Float>(t14554: F, t14558: F, t14563: F, t14060: F, t14081: F, t14229: F, t14233: F, t14556: F, t14560: F, t14568: F, t14571: F, t15049: F, t15060: F, t15071: F, t898: F, t338: F, t353: F) -> (F, F, F) {
    let t15072 = 7.0 / 144.0 * t14554;
    let t15074 = 7.0 / 288.0 * t14558;
    let t15076 = 7.0 / 72.0 * t14563;
    let t15079 = t15072 - t14556 / 192.0 + t15074 - t14560 / 96.0 + t14060 + t15076 + t14568 / 48.0 - t14571 / 48.0 + t14229 + t14081 + t14233;
    let t15081 = t15049 + t15060 + t15071 + t15079;
    let t15082 = t898 * t15081;
    let t15084 = t338 * t353 * t15082;
    (t15081, t15082, t15084)
}
