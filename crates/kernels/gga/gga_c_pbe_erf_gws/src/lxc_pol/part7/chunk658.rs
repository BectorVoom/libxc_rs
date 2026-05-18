//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 658/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk658<F: Float>(t641: F, t649: F, t617: F, t1816: F, t5211: F, t108: F, t560: F, t267: F) -> (F, F, F, F, F, F) {
    let t5212 = t641 * t649;
    let t5213 = t5212 * t617;
    let t5214 = t5213 * t1816;
    let t5216 = F::new(16.0) / F::new(15.0) * t5211 * t5214;
    let t5217 = t560 * t108;
    let t5218 = t5217 * t267;
    (t5212, t5213, t5214, t5216, t5217, t5218)
}
