//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1149/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1149<F: Float>(t2494: F, t4066: F, t4233: F, t945: F, t810: F, t4209: F, t4414: F, t1115: F, t14198: F, t14311: F, t14416: F, t14426: F, t14444: F, t14457: F, t14464: F, t14467: F, t14470: F, t2498: F, t3040: F, t4083: F) -> (F, F, F, F) {
    let t14849 = t4066 * t2494;
    let t14852 = t4233 * t945;
    let t14854 = t14852 * t810;
    let t14867 = t4414 * t4209;
    let t14873 = -t2498 * t4083 / F::new(96.0) - t1115 * t14311 / F::new(96.0) - t14416 / F::new(768.0) - t14426 / F::new(768.0) - t3040 * t4083 / F::new(96.0) + F::new(7.0) / F::new(288.0) * t14198 + t14444 / F::new(1536.0) - F::new(7.0) / F::new(144.0) * t14867 + t14457 / F::new(384.0) - t14464 / F::new(24.0) - t14467 / F::new(24.0) - t14470 / F::new(24.0);
    (t14849, t14852, t14854, t14873)
}
