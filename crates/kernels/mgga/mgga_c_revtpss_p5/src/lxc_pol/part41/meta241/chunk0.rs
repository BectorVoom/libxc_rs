//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 923/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk923<F: Float>(t300: F, t6541: F, t6514: F, t1765: F, t5192: F, t1188: F, t3495: F, t6518: F, t1196: F, t1179: F, t6534: F, t3520: F) -> (F, F, F, F, F, F, F, F) {
    let t6542 = t300 * t6541;
    let t6544 = F::cast_from(0.19751673498613801407e-1_f64) * t300 * t6514;
    let t6546 = F::cast_from(0.11696447245269292414e1_f64) * t5192 * t1765;
    let t6548 = t3495 * t6518 * t1188;
    let t6550 = F::cast_from(0.11696447245269292414e1_f64) * t1196 * t6548;
    let t6552 = t1179 * t6534 * t1188;
    let t6554 = F::cast_from(0.5848223622634646207e0_f64) * t1196 * t6552;
    let t6555 = t3520 * t6518;
    (t6542, t6544, t6546, t6548, t6550, t6552, t6554, t6555)
}
