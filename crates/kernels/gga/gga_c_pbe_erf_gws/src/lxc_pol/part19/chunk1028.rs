//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1028/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1028<F: Float>(t14852: F, t810: F, t4209: F, t4414: F, t1115: F, t14198: F, t14311: F, t14416: F, t14426: F, t14444: F, t14457: F, t14464: F, t14467: F, t14470: F, t2498: F, t3040: F, t4083: F) -> (F, F, F) {
    let t14854 = t14852 * t810;
    let t14867 = t4414 * t4209;
    let t14873 = -t2498 * t4083 / 96.0 - t1115 * t14311 / 96.0 - t14416 / 768.0 - t14426 / 768.0 - t3040 * t4083 / 96.0 + 7.0 / 288.0 * t14198 + t14444 / 1536.0 - 7.0 / 144.0 * t14867 + t14457 / 384.0 - t14464 / 24.0 - t14467 / 24.0 - t14470 / 24.0;
    (t14854, t14867, t14873)
}
