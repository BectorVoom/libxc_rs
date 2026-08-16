//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1284/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1284(t3358: f64, t3415: f64, t3459: f64, t3466: f64, t5044: f64, t5049: f64, t5054: f64, t5058: f64, t5072: f64, t5080: f64, t5088: f64, t5090: f64, t5093: f64, t5096: f64, t5099: f64, t5102: f64) -> f64 {
    let t5142 = -0.17648625e1_f64 * t5072 + 0.3529725e1_f64 * t5080 + t3459 - 0.17215833333333333333e0_f64 * t3358 - 0.17215833333333333333e0_f64 * t5044 - 0.34431666666666666667e0_f64 * t5049 + 0.103295e1_f64 * t5054 + 0.516475e0_f64 * t5058 + 0.31558125e0_f64 * t5088 + 0.6311625e0_f64 * t5090 + t3466 - 0.69463333333333333333e-1_f64 * t3415 - 0.69463333333333333333e-1_f64 * t5093 - 0.34731666666666666667e-1_f64 * t5096 + 0.20839e0_f64 * t5099 + 0.104195e0_f64 * t5102;
    t5142
}
