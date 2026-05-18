//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 930/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk930<F: Float>(t1182: F, t618: F, t1184: F, t236: F, t7231: F, t7365: F, t3352: F, t38928: F, t558: F, t1971: F, t511: F, t35190: F, t8450: F) -> (F, F, F, F, F, F) {
    let t40433 = t618 * t1182;
    let t40437 = t7365 * t7231 * t236 * t40433 * t1184;
    let t40442 = t7365 * t3352 * t236 * t38928 * t1184;
    let t40444 = t558 * t1182;
    let t40448 = t7365 * t1971 * t511 * t40444 * t1184;
    let t40450 = t8450 * t35190;
    (t40433, t40437, t40442, t40444, t40448, t40450)
}
