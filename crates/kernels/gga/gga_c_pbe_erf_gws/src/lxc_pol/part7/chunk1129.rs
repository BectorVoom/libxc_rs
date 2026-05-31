//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1129/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1129<F: Float>(t6: F, t6439: F, t2331: F, t362: F, t915: F, t2250: F, t2259: F, t2122: F, t2182: F, t337: F, t6560: F, t2120: F) -> (F, F, F, F, F) {
    let t20264 = t6 * t6439;
    let t20269 = t362 * t2331;
    let t20270 = t20269 * t915;
    let t20271 = t2250 * t20270;
    let t20272 = t20271 * t2259;
    let t20276 = t6560 * t337 * t2122 * t2182;
    let t20278 = F::cast_from(3.0_f64) / F::cast_from(8.0_f64) * t2120 * t20276;
    (t20264, t20269, t20270, t20272, t20278)
}
