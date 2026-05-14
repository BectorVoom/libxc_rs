//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 915/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk915<F: Float>(t2484: F, t7624: F, t7627: F, t815: F, t2150: F, t2491: F, t2490: F, t774: F, t755: F, t2526: F, t2622: F, t62: F, t157: F, t26521: F, t26523: F, t26525: F, t26528: F, t26531: F, t26534: F, t26536: F, t26538: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t26540 = t2484 * t7624;
    let t26542 = t815 * t7627;
    let t26544 = t2150 * t2491;
    let t26545 = t2490 * t26544;
    let t26547 = t7627 * t774;
    let t26548 = t755 * t26547;
    let t26550 = t2150 * t2526;
    let t26551 = t755 * t26550;
    let t26553 = t62 * t2622;
    let t26554 = t157 * t26553;
    let t26556 = 0.1875e0 * t26521 - 0.375e0 * t26523 - 0.75e0 * t26525 + 0.375e0 * t26528 + 0.75e0 * t26531 - 0.1875e0 * t26534 + 0.1125e1 * t26536 - 0.809375e-1 * t26538 + 0.161875e0 * t26540 + 0.6475e0 * t26542 - 0.161875e0 * t26545 - 0.6475e0 * t26548 + 0.809375e-1 * t26551 - 0.161875e1 * t26554;
    (t26540, t26542, t26544, t26545, t26547, t26548, t26550, t26551, t26553, t26554, t26556)
}
