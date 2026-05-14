//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1080/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1080<F: Float>(t52961: F, t13808: F, t14588: F, t13772: F, t3083: F, t14437: F, t2367: F, t1113: F, t29103: F, t3972: F, t3975: F, t14397: F, t14791: F, t2388: F, t2392: F, t2408: F, t3040: F, t50927: F, t52940: F, t52944: F, t52952: F, t52956: F, t52959: F, t9218: F, t9283: F) -> (F,) {
    let t52962 = 7.0 / 2304.0 * t52961;
    let t52968 = t13808 * t14588;
    let t52969 = 7.0 / 1152.0 * t52968;
    let t52971 = 7.0 / 144.0 * t3083 * t13772;
    let t52973 = 7.0 / 144.0 * t2367 * t14437;
    let t52976 = t3972 * t3975 * t1113 * t29103;
    let t52978 = -t2388 * t14437 / 96.0 + t52940 / 384.0 + t52944 / 768.0 + t2408 * t9283 * t14791 * t9218 / 8.0 - t52952 / 3072.0 + t52956 / 768.0 - t52959 / 192.0 - t52962 + 7.0 / 1152.0 * t50927 - t3040 * t13772 / 48.0 - t2392 * t14397 / 96.0 + t52969 + t52971 + t52973 + t52976 / 768.0;
    (t52978,)
}
