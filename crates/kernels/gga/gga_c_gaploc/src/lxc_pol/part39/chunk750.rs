//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 750/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk750<F: Float>(t3689: F, t447: F, t2366: F, t475: F, t6508: F, t12000: F, t158: F, t599: F, t203: F, t1: F, t544: F, t1564: F, t1359: F, t12064: F, t540: F, t106: F, t192: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t38271 = t3689 * t447;
    let t38272 = t2366 * t38271;
    let t38276 = t3689 * t475;
    let t38277 = t6508 * t38276;
    let t38281 = t2366 * t38276;
    let t38285 = t158 * t12000;
    let t38392 = t599 * t12000;
    let t38413 = t203 * t12000;
    let t38486 = t544 * t38285 * t1;
    let t38613 = t1564 * t12000;
    let t38674 = t1359 * t3689;
    let t38688 = t12064 * t540;
    let t38759 = t12000 * t1 * t106 * t192;
    (t38272, t38277, t38281, t38392, t38413, t38486, t38613, t38674, t38688, t38759)
}
