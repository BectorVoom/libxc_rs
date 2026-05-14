//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 856/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk856<F: Float>(t1049: F, t1986: F, t231: F, t4910: F, t7002: F, t7007: F, t7008: F, t7009: F, t7010: F, t7013: F, t7015: F, t7017: F, t7023: F, t7026: F, t7031: F, t7033: F, t8400: F, t8404: F) -> (F,) {
    let t8405 = t1049 * t1986;
    let t8407 = t7002 - t7007 + t7008 + t4910 + t7009 + 4.0 / 3.0 * t8400 * t231 + t8404 + 4.0 / 3.0 * t8405 + t7010 - t7013 + t7015 - t7017 + t7023 - t7026 + t7031 - t7033;
    (t8407,)
}
