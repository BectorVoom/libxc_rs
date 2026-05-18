//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1204/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1204<F: Float>(t1168: F, t13086: F, t18512: F, t18518: F, t19482: F, t2429: F, t3703: F, t3717: F, t3718: F, t3929: F, t3932: F, t47181: F, t48441: F, t48442: F, t48443: F, t48444: F, t48445: F, t48446: F, t48474: F, t48475: F, t48478: F, t804: F) -> F {
    let t48948 = F::new(12.0) * t1168 * t13086 * t804 + F::new(36.0) * t2429 * t3703 * t3929 - F::new(18.0) * t3717 * t3932 * t804 + F::new(36.0) * t3718 * t47181 + t18512 + t18518 + t19482 + t48441 + t48442 - t48443 + t48444 + t48445 - t48446 + t48474 - t48475 - t48478;
    t48948
}
