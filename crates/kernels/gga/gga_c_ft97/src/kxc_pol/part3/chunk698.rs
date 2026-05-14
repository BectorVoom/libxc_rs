//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 698/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk698<F: Float>(t16246: F, t492: F, t83: F, t4551: F, t8466: F, t1882: F, t4617: F, t3238: F, t3271: F, t452: F, t432: F, t4495: F, t110: F, t1871: F, t488: F, t3266: F, t986: F) -> (F, F, F, F, F, F, F, F, F) {
    let t16247 = t16246 * t492;
    let t16248 = t83 * t16247;
    let t16251 = t8466 * t4551;
    let t16252 = t83 * t16251;
    let t16255 = t1882 * t4617;
    let t16258 = t452 * t3238 * t3271;
    let t16261 = t4495 * t432;
    let t16263 = t1871 * t110 * t16261;
    let t16266 = t4495 * t492;
    let t16268 = t452 * t488 * t16266;
    let t16272 = t1871 * t986 * t3266;
    (t16247, t16248, t16251, t16252, t16255, t16258, t16263, t16268, t16272)
}
