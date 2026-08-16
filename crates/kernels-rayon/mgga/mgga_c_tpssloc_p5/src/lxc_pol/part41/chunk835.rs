//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 835/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk835(t1137: f64, t6036: f64, t3339: f64, t3346: f64, t4721: f64, t4770: f64, t5973: f64, t5977: f64, t5981: f64, t5993: f64, t6000: f64, t6006: f64, t6008: f64, t6012: f64, t6015: f64, t6018: f64) -> (f64, f64) {
    let t6037 = t6036 * t1137;
    let t6052 = -0.17648625e1_f64 * t5993 + 0.3529725e1_f64 * t6000 + t3339 - 0.34431666666666666666e0_f64 * t4721 - 0.34431666666666666667e0_f64 * t5973 + 0.103295e1_f64 * t5977 + 0.516475e0_f64 * t5981 + 0.31558125e0_f64 * t6006 + 0.6311625e0_f64 * t6008 + t3346 - 0.13892666666666666667e0_f64 * t4770 - 0.34731666666666666667e-1_f64 * t6012 + 0.20839e0_f64 * t6015 + 0.104195e0_f64 * t6018;
    (t6037, t6052)
}
