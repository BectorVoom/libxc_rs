//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1128/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1128<F: Float>(t14011: F, t3237: F, t3120: F, t4023: F, t14031: F, t3228: F, t14069: F, t3123: F, t367: F, t6238: F, t899: F) -> (F, F, F, F, F) {
    let t14489 = t14011 * t3237;
    let t14491 = t3120 * t4023;
    let t14493 = t14031 * t3228;
    let t14495 = t3123 * t14069;
    let t14498 = t899 * t6238 * t367;
    (t14489, t14491, t14493, t14495, t14498)
}
