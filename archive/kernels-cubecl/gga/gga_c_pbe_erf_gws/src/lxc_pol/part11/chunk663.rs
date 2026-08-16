//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 663/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk663<F: Float>(t329: F, t369: F, t838: F, t2052: F, t381: F, t2096: F, t2454: F, t4: F, t959: F) -> (F, F, F, F) {
    let t6832 = t329 * t838 * t369;
    let t6854 = F::cast_from(1.0_f64) / t2052 / t381;
    let t6906 = t2454 * t2096;
    let t6967 = t959 * t4;
    (t6832, t6854, t6906, t6967)
}
