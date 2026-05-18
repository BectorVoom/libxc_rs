//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 669/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk669<F: Float>(t2157: F, t609: F, t2179: F, t144: F, t2075: F, t558: F, t167: F, t2185: F, t571: F, t8232: F, t1882: F, t2192: F) -> (F, F, F, F, F, F, F) {
    let t9288 = t609 * t2157;
    let t9289 = t2179 * t9288;
    let t9290 = t144 * t9289;
    let t9293 = t2075 * t558;
    let t9295 = t2185 * t167 * t9293;
    let t9298 = t8232 * t571;
    let t9300 = t1882 * t2192;
    (t9288, t9289, t9290, t9293, t9295, t9298, t9300)
}
