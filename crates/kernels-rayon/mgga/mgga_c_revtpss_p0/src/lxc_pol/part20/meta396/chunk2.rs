//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1459/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1459(t41306: f64, t41292: f64, t41299: f64, t41303: f64, t41341: f64, t41344: f64, t41347: f64, t41350: f64, t41361: f64, t41363: f64, t41369: f64, t41373: f64, t41384: f64, t41387: f64) -> f64 {
    let t41610 = 0.31003950617283950618e1_f64 * t41306;
    let t41621 = 0.97370864197530864196e-1_f64 * t41292 - 0.85199506172839506175e-1_f64 * t41299 - 0.82156666666666666667e-1_f64 * t41303 + t41610 + 0.3071625e0_f64 * t41373 - 0.88582716049382716048e0_f64 * t41341 - 0.29896666666666666667e0_f64 * t41344 - 0.71752000000000000002e1_f64 * t41347 + 0.39862222222222222223e1_f64 * t41350 + 0.12401580246913580247e1_f64 * t41361 + 0.15944888888888888889e1_f64 * t41363 - 0.15944888888888888889e1_f64 * t41369 + 0.1151859375e0_f64 * t41384 + 0.46074375e0_f64 * t41387;
    t41621
}
