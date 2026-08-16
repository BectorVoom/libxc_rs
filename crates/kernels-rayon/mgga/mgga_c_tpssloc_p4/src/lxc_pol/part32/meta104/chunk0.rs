//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 657/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk657(t2388: f64, t2391: f64, t2394: f64, t2398: f64, t2400: f64, t2403: f64, t702: f64, t683: f64) -> (f64, f64, f64) {
    let t2405 = -0.42198333333333333333e0_f64 * t2388 + 0.84396666666666666666e0_f64 * t2391 + 0.39862222222222222223e0_f64 * t2394 + 0.68258333333333333333e-1_f64 * t2398 + 0.13651666666666666667e0_f64 * t2400 + 0.13692777777777777778e0_f64 * t2403;
    let t2406 = t2405 * t702;
    let t2408 = 1.0_f64 * t683 * t2406;
    (t2405, t2406, t2408)
}
