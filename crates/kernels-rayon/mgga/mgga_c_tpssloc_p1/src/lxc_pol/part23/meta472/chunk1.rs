//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1409/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1409(t63332: f64, t63334: f64, t63888: f64, t63893: f64, t63911: f64, t71142: f64, t71144: f64, t71146: f64, t71152: f64, t71154: f64, t71156: f64, t71408: f64, t78002: f64, t78005: f64) -> f64 {
    let t78019 = 0.40256666666666666666e1_f64 * t78002 - 0.60384999999999999999e0_f64 * t78005 - 0.53675555555555555556e0_f64 * t63332 + 0.80513333333333333336e0_f64 * t63334 - 0.18396666666666666667e0_f64 * t63888 + 0.11038e1_f64 * t63893 + 0.80513333333333333333e0_f64 * t71142 - 0.24154e1_f64 * t71144 + 0.5519e0_f64 * t63911 - 0.22076e0_f64 * t71408 - 0.44729629629629629629e0_f64 * t71146 - 0.24154e1_f64 * t71152 - 0.40256666666666666668e0_f64 * t71154 + 0.16102666666666666667e1_f64 * t71156;
    t78019
}
