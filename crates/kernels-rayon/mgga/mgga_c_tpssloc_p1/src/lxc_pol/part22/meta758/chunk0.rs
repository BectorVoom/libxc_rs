//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2544/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2544(t1113: f64, t136: f64, t71189: f64, t71201: f64, t71191: f64, t71195: f64, t71199: f64, t71468: f64, t71470: f64, t71472: f64, t71474: f64, t71477: f64, t71480: f64, t71483: f64) -> (f64, f64, f64) {
    let t71486 = t136 * t1113 * t71189;
    let t71489 = t136 * t1113 * t71201;
    let t71494 = 0.247573125e0_f64 * t71468 - 0.24528888888888888889e-1_f64 * t71470 + 0.11038e0_f64 * t71472 - 0.33114e0_f64 * t71474 + 0.16557e0_f64 * t71477 - 0.82785e-1_f64 * t71480 - 0.82785e-1_f64 * t71483 + 0.49671e0_f64 * t71486 + 0.49671e0_f64 * t71489 + 0.181155e1_f64 * t71191 - 0.36231e1_f64 * t71195 - 0.72462e1_f64 * t71199;
    (t71486, t71489, t71494)
}
