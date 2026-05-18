//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 867/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk867<F: Float>(t4903: F, t4913: F, t5156: F, t16630: F, t16633: F, t16636: F, t16639: F, t16642: F, t16645: F, t16648: F, t16651: F, t16653: F) -> (F, F, F) {
    let t16655 = F::new(32.0) / F::new(15.0) * t4913 * t4903;
    let t16657 = F::new(32.0) / F::new(9.0) * t4913 * t5156;
    let t16658 = t16630 - t16633 - t16636 - t16639 + t16642 + t16645 - t16648 + t16651 + t16653 + t16655 + t16657;
    (t16655, t16657, t16658)
}
