//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1282/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1282<F: Float>(t3147: F, t6226: F, t6502: F, t6320: F, t8219: F, t2313: F, t8020: F, t898: F, t1184: F, t2240: F, t6327: F, t237: F, t6323: F) -> (F, F, F, F, F, F) {
    let t22492 = F::new(0.5848223622634646207e0) * t3147 * t6226;
    let t22494 = F::new(0.35089341735807877242e1) * t3147 * t6502;
    let t22496 = F::new(6.0) * t8219 * t6320;
    let t22499 = F::new(0.35089341735807877242e1) * t898 * t8020 * t2313;
    let t22500 = t2240 * t1184;
    let t22502 = F::new(18.0) * t22500 * t6327;
    let t22503 = t237 * t6323;
    (t22492, t22494, t22496, t22499, t22502, t22503)
}
