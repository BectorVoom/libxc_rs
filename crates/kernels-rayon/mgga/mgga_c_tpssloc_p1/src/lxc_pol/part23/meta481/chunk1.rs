//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1440/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1440(t3447: f64, t4900: f64, t4904: f64, t64821: f64, t73169: f64, t73330: f64, t73386: f64, t73389: f64, t73395: f64, t73417: f64, t73420: f64, t73424: f64, t78031: f64, t78039: f64) -> f64 {
    let t78460 = 0.11111111111111111111e-2_f64 * t3447 * t73169 * t4904 - 0.22222222222222222221e-2_f64 * t73330 + 0.88888888888888888887e-2_f64 * t73386 - 0.11111111111111111111e-2_f64 * t73389 + 0.11111111111111111111e-2_f64 * t73395 - 0.14814814814814814815e-2_f64 * t73417 + 0.11111111111111111111e-2_f64 * t73420 - 0.74074074074074074072e-3_f64 * t64821 + 0.88888888888888888887e-2_f64 * t73424 + 0.14814814814814814815e-2_f64 * t3447 * t4900 * t78031 + 0.13333333333333333333e-1_f64 * t3447 * t4900 * t78039;
    t78460
}
