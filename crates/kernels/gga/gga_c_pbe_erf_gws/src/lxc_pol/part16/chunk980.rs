//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 980/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk980<F: Float>(t14031: F, t2259: F, t366: F, t6238: F, t899: F, t2268: F, t2173: F, t4028: F, t1184: F, t2216: F, t4033: F, t888: F, t360: F, t56: F, t837: F, t863: F) -> (F, F, F, F, F, F, F) {
    let t14032 = t14031 * t2259;
    let t14035 = t899 * t6238 * t366;
    let t14036 = t14035 * t2268;
    let t14038 = t4028 * t2173;
    let t14040 = t1184 * t2216;
    let t14042 = t4033 * t888;
    let t14043 = 7.0 / 72.0 * t14042;
    let t14046 = t863 * t360 * t837 * t56;
    (t14032, t14036, t14038, t14040, t14042, t14043, t14046)
}
