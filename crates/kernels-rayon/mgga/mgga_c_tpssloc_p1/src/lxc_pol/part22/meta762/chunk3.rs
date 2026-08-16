//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2567/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2567(t63332: f64, t63334: f64, t63336: f64, t63886: f64, t63888: f64, t63893: f64, t71124: f64, t71130: f64, t71135: f64, t71140: f64, t71142: f64, t71391: f64) -> f64 {
    let t71915 = 0.17215833333333333333e1_f64 * t71124 - 0.45908888888888888888e0_f64 * t63332 + 0.68863333333333333332e0_f64 * t63334 - 0.51647499999999999999e0_f64 * t63336 - 0.61977e1_f64 * t71130 - 0.20839e0_f64 * t63886 - 0.11577222222222222223e0_f64 * t63888 + 0.69463333333333333335e0_f64 * t63893 + 0.6311625e0_f64 * t71391 + 0.68863333333333333334e1_f64 * t71135 - 0.34431666666666666667e0_f64 * t71140 + 0.34431666666666666667e0_f64 * t71142;
    t71915
}
