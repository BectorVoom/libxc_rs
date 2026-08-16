//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1099/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1099<F: Float>(t38885: F, t977: F, t42487: F, t42491: F, t42494: F, t42496: F, t42501: F, t42503: F, t43353: F, t43355: F, t44194: F, t47074: F, t47096: F) -> F {
    let t47097 = t38885 * t977;
    let t47098 = -t42487 - t42491 - t42494 - t42496 + t43353 - t43355 + t47074 - t42501 - t42503 - t44194 - t47096 - t47097;
    t47098
}
