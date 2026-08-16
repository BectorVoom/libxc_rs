//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2539/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2539(t136: f64, t3297: f64, t71193: f64, t71197: f64, t1113: f64, t71168: f64, t71172: f64, t63911: f64, t71144: f64, t71400: f64, t71403: f64, t71406: f64, t71408: f64, t71411: f64, t71414: f64) -> (f64, f64, f64, f64, f64) {
    let t71417 = t136 * t3297 * t71193;
    let t71420 = t136 * t3297 * t71197;
    let t71423 = t136 * t1113 * t71168;
    let t71426 = t136 * t1113 * t71172;
    let t71428 = -0.60385e0_f64 * t71144 - 0.8585111111111111111e-1_f64 * t71400 + 0.27595e0_f64 * t63911 + 0.258925e1_f64 * t71403 + 0.82785e-1_f64 * t71406 - 0.5519e-1_f64 * t71408 + 0.11038e0_f64 * t71411 + 0.44152e0_f64 * t71414 - 0.49671e0_f64 * t71417 - 0.99342e0_f64 * t71420 + 0.149013e1_f64 * t71423 + 0.198684e1_f64 * t71426;
    (t71417, t71420, t71423, t71426, t71428)
}
