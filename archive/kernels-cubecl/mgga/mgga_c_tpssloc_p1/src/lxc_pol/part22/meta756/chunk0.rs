//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2539/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2539<F: Float>(t136: F, t3297: F, t71193: F, t71197: F, t1113: F, t71168: F, t71172: F, t63911: F, t71144: F, t71400: F, t71403: F, t71406: F, t71408: F, t71411: F, t71414: F) -> (F, F, F, F, F) {
    let t71417 = t136 * t3297 * t71193;
    let t71420 = t136 * t3297 * t71197;
    let t71423 = t136 * t1113 * t71168;
    let t71426 = t136 * t1113 * t71172;
    let t71428 = -F::cast_from(0.60385e0_f64) * t71144 - F::cast_from(0.8585111111111111111e-1_f64) * t71400 + F::cast_from(0.27595e0_f64) * t63911 + F::cast_from(0.258925e1_f64) * t71403 + F::cast_from(0.82785e-1_f64) * t71406 - F::cast_from(0.5519e-1_f64) * t71408 + F::cast_from(0.11038e0_f64) * t71411 + F::cast_from(0.44152e0_f64) * t71414 - F::cast_from(0.49671e0_f64) * t71417 - F::cast_from(0.99342e0_f64) * t71420 + F::cast_from(0.149013e1_f64) * t71423 + F::cast_from(0.198684e1_f64) * t71426;
    (t71417, t71420, t71423, t71426, t71428)
}
