//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 822/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk822<F: Float>(t43: F, t338: F, t3737: F, t892: F, t3887: F, t3342: F, t4757: F, t1402: F, t3346: F, t1351: F, t2457: F, t418: F, t47: F, t9788: F, t3351: F, t4767: F, t1412: F, t3354: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t44 = t43 <= zeta_threshold;
    let t9973 = t338 * t892 * t3737;
    let t9978 = t338 * t892 * t3887;
    let t9981 = t4757 * t3342;
    let t9986 = t1402 * t3346;
    let t9992 = piecewise3(t44, 0.0, -8.0 / 27.0 * t9981 * t418 + 16.0 / 9.0 * t2457 * t1351 + 4.0 / 9.0 * t9986 * t418 + 4.0 / 3.0 * t47 * t9788);
    let t9993 = t4767 * t3351;
    let t9998 = t1412 * t3354;
    (t9973, t9978, t9992, t9993, t9998)
}
