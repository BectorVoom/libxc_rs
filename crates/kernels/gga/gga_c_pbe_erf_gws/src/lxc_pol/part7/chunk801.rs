//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 801/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk801<F: Float>(t16647: F, t5342: F, t586: F, t1812: F, t4913: F, t5142: F, t4903: F, t5156: F, t16630: F, t16633: F, t16636: F, t16639: F, t16642: F, t16645: F, t4929: F, t5211: F, t617: F, t7116: F) -> (F, F, F, F, F, F, F, F) {
    let t16648 = 32.0 / 45.0 * t16647;
    let t16649 = t5342 * t586;
    let t16651 = 32.0 / 15.0 * t16649 * t1812;
    let t16653 = 32.0 / 15.0 * t4913 * t5142;
    let t16655 = 32.0 / 15.0 * t4913 * t4903;
    let t16657 = 32.0 / 9.0 * t4913 * t5156;
    let t16658 = t16630 - t16633 - t16636 - t16639 + t16642 + t16645 - t16648 + t16651 + t16653 + t16655 + t16657;
    let t16662 = 64.0 / 15.0 * t5211 * t7116 * t617 * t4929;
    (t16648, t16649, t16651, t16653, t16655, t16657, t16658, t16662)
}
