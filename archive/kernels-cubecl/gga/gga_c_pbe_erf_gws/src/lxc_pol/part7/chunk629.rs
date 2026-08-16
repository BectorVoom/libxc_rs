//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 629/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk629<F: Float>(t211: F, t4908: F, t1750: F, t636: F, t1729: F, t586: F) -> (F, F, F) {
    let t4910 = F::cast_from(16.0_f64) / F::cast_from(405.0_f64) * t211 * t4908;
    let t4911 = t1750 * t636;
    let t4912 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t4911;
    let t4913 = t1729 * t586;
    (t4910, t4912, t4913)
}
