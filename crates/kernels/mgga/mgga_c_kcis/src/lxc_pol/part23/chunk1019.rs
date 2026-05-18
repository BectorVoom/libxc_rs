//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1019/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1019<F: Float>(t2490: F, t26544: F, t7627: F, t774: F, t755: F, t2150: F, t2526: F, t2622: F, t62: F, t157: F, t26521: F, t26523: F, t26525: F, t26528: F, t26531: F, t26534: F, t26536: F, t26538: F, t26540: F, t26542: F) -> (F, F, F, F, F, F, F, F) {
    let t26545 = t2490 * t26544;
    let t26547 = t7627 * t774;
    let t26548 = t755 * t26547;
    let t26550 = t2150 * t2526;
    let t26551 = t755 * t26550;
    let t26553 = t62 * t2622;
    let t26554 = t157 * t26553;
    let t26556 = F::new(0.1875e0) * t26521 - F::new(0.375e0) * t26523 - F::new(0.75e0) * t26525 + F::new(0.375e0) * t26528 + F::new(0.75e0) * t26531 - F::new(0.1875e0) * t26534 + F::new(0.1125e1) * t26536 - F::new(0.809375e-1) * t26538 + F::new(0.161875e0) * t26540 + F::new(0.6475e0) * t26542 - F::new(0.161875e0) * t26545 - F::new(0.6475e0) * t26548 + F::new(0.809375e-1) * t26551 - F::new(0.161875e1) * t26554;
    (t26545, t26547, t26548, t26550, t26551, t26553, t26554, t26556)
}
