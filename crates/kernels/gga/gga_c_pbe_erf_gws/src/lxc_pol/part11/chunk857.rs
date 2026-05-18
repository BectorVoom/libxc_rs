//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 857/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk857<F: Float>(t11846: F, t11852: F, t11857: F, t11864: F, t13456: F, t13457: F, t13459: F, t13465: F, t13470: F, t13475: F, t13478: F, t13479: F, t13481: F, t902: F) -> F {
    let t13484 = t13456 - t13457 - t13459 - F::new(7.0) / F::new(256.0) * t11846 - t13465 + F::new(7.0) / F::new(192.0) * t11852 + t13470 - F::new(7.0) / F::new(96.0) * t11857 - t13475 - F::new(7.0) / F::new(384.0) * t11864 + t13478 + t13479 + t902 * t13481 / F::new(1536.0);
    t13484
}
