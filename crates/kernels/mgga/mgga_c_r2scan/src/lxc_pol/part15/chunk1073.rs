//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1073/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1073<F: Float>(t11020: F, t11483: F, t11626: F, t37271: F, t11476: F, t37282: F, t11519: F, t11563: F, t2312: F, t3446: F, t3447: F, t158: F, t2461: F, t874: F, t122: F, t3434: F, t3437: F) -> (F, F, F, F, F, F, F) {
    let t40443 = t11020 * t11483 / 4.0;
    let t40444 = t37271 * t11626;
    let t40446 = 3.0 / 2.0 * t37282 * t11476;
    let t40448 = 15.0 / 8.0 * t37282 * t11519;
    let t40451 = t3446 * t3447 * t11563 * t2312;
    let t40453 = t158 * t2461;
    let t40456 = t3446 * t3447 * t40453 * t874;
    let t40457 = 0.30487649791575028314e-3 * t40456;
    let t40460 = t3434 * t3437 * t40453 * t122;
    (t40443, t40444, t40446, t40448, t40451, t40457, t40460)
}
