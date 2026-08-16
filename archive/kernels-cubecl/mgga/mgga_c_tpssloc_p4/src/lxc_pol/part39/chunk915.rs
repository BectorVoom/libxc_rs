//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 915/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk915<F: Float>(t59: F, t9533: F, t212: F, t2386: F, t116: F, t131: F, t207: F, t2559: F, t786: F, t789: F, t2563: F, t2582: F) -> (F, F, F, F, F, F) {
    let t9534 = t59 * t9533;
    let t9537 = t2386 * t212;
    let t9538 = t116 * t131 * t9537;
    let t9540 = F::cast_from(0.13888888888888888889e-3_f64) * t9534 * t207 * t9538;
    let t9541 = t2559 * t786;
    let t9542 = t9541 * t789;
    let t9544 = t2563 * t2582;
    (t9534, t9538, t9540, t9541, t9542, t9544)
}
