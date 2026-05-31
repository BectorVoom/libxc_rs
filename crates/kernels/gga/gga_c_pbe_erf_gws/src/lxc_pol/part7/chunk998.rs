//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 998/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk998<F: Float>(t1914: F, t5421: F, t17328: F, t17330: F, t17335: F, t17338: F, t17341: F, t17343: F, t18240: F, t18243: F, t18245: F, t18247: F, t18250: F) -> F {
    let t18252 = t1914 * t5421;
    let t18254 = t17328 + t17330 - t17335 + t17338 + t17341 - t17343 + t18240 - t18243 - t18245 + F::cast_from(2.0_f64) * t18247 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t18250 + F::cast_from(0.72933333333333333331e0_f64) * t18252;
    t18254
}
