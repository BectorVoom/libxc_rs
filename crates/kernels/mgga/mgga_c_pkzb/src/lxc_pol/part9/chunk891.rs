//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 891/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk891<F: Float>(t6441: F, t6534: F, t158: F, t941: F, t2429: F, t951: F, t2428: F, t2453: F, t410: F, t6514: F, t5728: F, t6460: F) -> (F, F, F, F, F, F, F, F, F) {
    let t6535 = t6441 + t6534;
    let t6536 = t6535 * t158;
    let t6545 = t941 * t941;
    let t6546 = F::new(1.0) / t6545;
    let t6547 = t2429 * t951;
    let t6548 = t6546 * t6547;
    let t6551 = t2428 * t951;
    let t6552 = t6551 * t2453;
    let t6555 = t6514 * t410;
    let t6556 = t6460 * t5728;
    (t6535, t6536, t6545, t6546, t6547, t6548, t6552, t6555, t6556)
}
