//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1192/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1192<F: Float>(t14024: F, t2115: F, t2087: F, t13806: F, t2276: F, t932: F, t2315: F, t2118: F, t2132: F, t822: F, t2263: F, t331: F, t56: F, t863: F) -> (F, F, F, F, F, F, F) {
    let t51244 = t2115 * t14024;
    let t51252 = t2087 * t14024;
    let t51255 = t2276 * t13806 * t932;
    let t51256 = t51255 * t2315;
    let t51266 = t2118 * t2132;
    let t51267 = t822 * t51266;
    let t51274 = t863 * t2263 * t331 * t56;
    (t51244, t51252, t51255, t51256, t51266, t51267, t51274)
}
