//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 829/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk829<F: Float>(t20114: F, t375: F, t89: F, t20099: F, t1775: F, t20369: F, t20384: F, t20356: F, t20359: F, t20372: F, t20366: F, t20381: F, t20098: F, t358: F, t2: F, t20337: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t73439 = t89 * t375 * t20114;
    let t73442 = t89 * t375 * t20099;
    let t73497 = t1775 * t20369;
    let t73504 = t1775 * t20384;
    let t73506 = t1775 * t20356;
    let t73508 = t1775 * t20359;
    let t73574 = t1775 * t20372;
    let t73576 = t1775 * t20366;
    let t73637 = t1775 * t20381;
    let t73639 = t20098 * t358;
    let t73653 = t2 * t20098;
    let t73662 = t1775 * t20337;
    (t73439, t73442, t73497, t73504, t73506, t73508, t73574, t73576, t73637, t73639, t73653, t73662)
}
