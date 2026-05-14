//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 837/1212 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk837<F: Float>(t760: F, t9419: F, t2516: F, t2523: F, t9387: F, t2496: F, t189: F, t606: F, t2258: F, t4401: F, t9372: F, t37: F, t716: F, t2612: F, t2626: F, t9425: F) -> (F, F, F, F, F, F, F, F, F) {
    let t10592 = 0.10389515463408878255e3 * t760 * t9419;
    let t10593 = t2523 * t2516;
    let t10594 = 0.17544670867903938621e1 * t10593;
    let t10596 = 0.5848223622634646207e0 * t760 * t9387;
    let t10597 = t2523 * t2496;
    let t10598 = 0.51947577317044391276e2 * t10597;
    let t10599 = t189 * t606;
    let t10600 = t10599 * t2258;
    let t10602 = 36.0 * t4401 * t10600;
    let t10604 = 0.10254018858216406658e4 * t760 * t9372;
    let t10605 = t37 * t716;
    let t10607 = 36.0 * t10605 * t2612;
    let t10608 = t2523 * t2626;
    let t10609 = 0.35089341735807877242e1 * t10608;
    let t10611 = 0.35089341735807877242e1 * t760 * t9425;
    (t10592, t10594, t10596, t10598, t10602, t10604, t10607, t10609, t10611)
}
