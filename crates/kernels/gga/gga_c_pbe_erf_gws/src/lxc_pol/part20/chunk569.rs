//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 569/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk569<F: Float>(t2138: F, t3123: F, t1125: F, t2142: F, t1114: F, t2145: F) -> (F, F, F, F) {
    let t3125 = t3123 * t2138 / 96.0;
    let t3126 = t1125 * t2142;
    let t3127 = 7.0 / 288.0 * t3126;
    let t3128 = t1114 * t2145;
    (t3125, t3126, t3127, t3128)
}
