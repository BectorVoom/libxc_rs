//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 712/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk712<F: Float>(t343: F, t6177: F, t337: F, t2121: F, t2134: F, t2365: F, t828: F) -> (F, F, F, F) {
    let t6178 = t6177 * t343;
    let t6179 = t337 * t6178;
    let t6180 = t2121 * t6179;
    let t6182 = t2134 * t6180 / 32.0;
    let t6183 = t2365 * t828;
    (t6179, t6180, t6182, t6183)
}
