//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1029/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1029<F: Float>(t11529: F, t3258: F, t2255: F, t2257: F, t3781: F, t11492: F, t11494: F, t11495: F, t11497: F, t11502: F, t11506: F, t11513: F, t11516: F, t11519: F, t11521: F, t11526: F, t2253: F, t2277: F, t2312: F, t2343: F, t3247: F, t6579: F, t902: F) -> (F, F, F, F, F) {
    let t11530 = t3258 * t11529;
    let t11531 = t2255 * t11530;
    let t11534 = t3781 * t2257;
    let t11535 = t2255 * t11534;
    let t11538 = -t11492 + t11494 + F::cast_from(7.0_f64) / F::cast_from(2304.0_f64) * t11495 + F::cast_from(7.0_f64) / F::cast_from(2304.0_f64) * t11497 + t2277 * t11502 / F::cast_from(384.0_f64) + t902 * t11506 / F::cast_from(1536.0_f64) + t11513 + t2343 * t11516 / F::cast_from(384.0_f64) - t11519 + t3247 * t11521 / F::cast_from(512.0_f64) + F::cast_from(5.0_f64) / F::cast_from(192.0_f64) * t6579 * t11526 + t2312 * t11531 / F::cast_from(192.0_f64) - t2253 * t11535 / F::cast_from(768.0_f64);
    (t11530, t11531, t11534, t11535, t11538)
}
