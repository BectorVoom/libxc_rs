//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1014/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1014<F: Float>(t97156: F, t97176: F, t97207: F, t97209: F, t97214: F, t97235: F, t97238: F, t97247: F, t1882: F, t24707: F, t24660: F, t8392: F, t24811: F, t24596: F, t6085: F, t8232: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t97385 = 4.0 / 9.0 * t97156;
    let t97391 = t97176 / 8.0;
    let t97399 = t97207 / 9.0;
    let t97400 = 2.0 / 9.0 * t97209;
    let t97403 = 2.0 / 9.0 * t97214;
    let t97408 = t97235 / 6.0;
    let t97409 = t97238 / 12.0;
    let t97412 = 28.0 / 81.0 * t97247;
    let t97422 = t1882 * t24707;
    let t97424 = t8392 * t24660;
    let t97451 = t1882 * t24811;
    let t97463 = t1882 * t24596;
    let t97470 = t8232 * t6085;
    (t97385, t97391, t97399, t97400, t97403, t97408, t97409, t97412, t97422, t97424, t97451, t97463, t97470)
}
