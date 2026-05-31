//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1459/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1459<F: Float>(t41306: F, t41292: F, t41299: F, t41303: F, t41341: F, t41344: F, t41347: F, t41350: F, t41361: F, t41363: F, t41369: F, t41373: F, t41384: F, t41387: F) -> F {
    let t41610 = F::cast_from(0.31003950617283950618e1_f64) * t41306;
    let t41621 = F::cast_from(0.97370864197530864196e-1_f64) * t41292 - F::cast_from(0.85199506172839506175e-1_f64) * t41299 - F::cast_from(0.82156666666666666667e-1_f64) * t41303 + t41610 + F::cast_from(0.3071625e0_f64) * t41373 - F::cast_from(0.88582716049382716048e0_f64) * t41341 - F::cast_from(0.29896666666666666667e0_f64) * t41344 - F::cast_from(0.71752000000000000002e1_f64) * t41347 + F::cast_from(0.39862222222222222223e1_f64) * t41350 + F::cast_from(0.12401580246913580247e1_f64) * t41361 + F::cast_from(0.15944888888888888889e1_f64) * t41363 - F::cast_from(0.15944888888888888889e1_f64) * t41369 + F::cast_from(0.1151859375e0_f64) * t41384 + F::cast_from(0.46074375e0_f64) * t41387;
    t41621
}
