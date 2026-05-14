//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 711/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk711<F: Float>(t2353: F, t338: F, t892: F, t2420: F, t840: F, t2355: F, t2156: F, t5: F) -> (F, F, F, F) {
    let t6170 = t338 * t892 * t2353;
    let t6173 = t840 * t2420;
    let t6175 = t840 * t2355;
    let t6177 = t5 * t2156;
    (t6170, t6173, t6175, t6177)
}
