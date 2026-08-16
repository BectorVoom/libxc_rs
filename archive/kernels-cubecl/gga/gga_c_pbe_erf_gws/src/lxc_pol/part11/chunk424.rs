//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 424/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk424<F: Float>(t359: F, t362: F) -> (F, F, F) {
    let t2262 = t359 * t359;
    let t2263 = F::cast_from(1.0_f64) / t2262;
    let t2264 = t2263 * t362;
    (t2262, t2263, t2264)
}
