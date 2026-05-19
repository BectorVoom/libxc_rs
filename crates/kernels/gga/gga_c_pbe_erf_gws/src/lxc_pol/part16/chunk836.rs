//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 836/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk836<F: Float>(t267: F, t4872: F, t4873: F, t4876: F, t4910: F, t6971: F, t6995: F, t6998: F, t7002: F, t7007: F, t7008: F, t7009: F, t7010: F, t7013: F, t7015: F, t7017: F, t7023: F) -> F {
    let t7024 = -t4872 - t6971 - t6995 * t267 / F::new(15.0) + F::new(2.0) / F::new(135.0) * t6998 + F::cast_from(0.66490888888888888888e-1_f64) * t4873 + t4876 + t7002 - t7007 + t7008 + t4910 + t7009 + t7010 - t7013 + t7015 - t7017 + t7023;
    t7024
}
