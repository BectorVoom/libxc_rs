//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1007/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1007<F: Float>(t760: F, t9419: F, t2516: F, t2523: F, t9387: F, t2496: F, t189: F, t606: F, t2258: F, t4401: F, t9372: F, t37: F, t716: F) -> (F, F, F, F, F, F, F, F, F) {
    let t10592 = F::cast_from(0.10389515463408878255e3_f64) * t760 * t9419;
    let t10593 = t2523 * t2516;
    let t10594 = F::cast_from(0.17544670867903938621e1_f64) * t10593;
    let t10596 = F::cast_from(0.5848223622634646207e0_f64) * t760 * t9387;
    let t10597 = t2523 * t2496;
    let t10598 = F::cast_from(0.51947577317044391276e2_f64) * t10597;
    let t10599 = t189 * t606;
    let t10600 = t10599 * t2258;
    let t10602 = F::cast_from(36.0_f64) * t4401 * t10600;
    let t10604 = F::cast_from(0.10254018858216406658e4_f64) * t760 * t9372;
    let t10605 = t37 * t716;
    (t10592, t10594, t10596, t10598, t10599, t10600, t10602, t10604, t10605)
}
