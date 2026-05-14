//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1000/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1000<F: Float>(t41966: F, t88252: F, t89: F, t9716: F, t666: F, t9749: F, t1131: F, t193: F, t80748: F, t1091: F, t21477: F, t446: F, t9770: F, t81095: F, t81102: F, t81105: F, t81124: F, t81131: F, t89047: F, t89051: F, t89054: F, t89058: F, t89062: F, t89069: F) -> (F, F, F, F, F, F) {
    let t89073 = t89 * t9716 * t41966 * t88252;
    let t89077 = t89 * t666 * t9749 * t88252;
    let t89081 = t89 * t193 * t80748 * t1131;
    let t89083 = t1091 * t21477;
    let t89085 = t446 * t9770 * t89083;
    let t89089 = -80.0 / 81.0 * t89047 - t89051 + 6.0 * t89054 + 24.0 * t89058 - t89062 / 3.0 + 8.0 / 3.0 * t81095 - 8.0 * t81102 + 4.0 / 9.0 * t81105 - 36.0 * t89069 + 40.0 / 9.0 * t89073 + 8.0 * t89077 + 8.0 * t89081 - 8.0 * t89085 + 4.0 / 3.0 * t81124 + 40.0 / 81.0 * t81131;
    (t89073, t89077, t89081, t89083, t89085, t89089)
}
