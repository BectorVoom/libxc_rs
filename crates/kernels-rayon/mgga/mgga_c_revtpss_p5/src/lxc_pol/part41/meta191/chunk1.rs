//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 777/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk777(t1179: f64, t1749: f64, t1187: f64, t1757: f64, t3358: f64, t3415: f64, t3503: f64, t3510: f64, t5044: f64, t5049: f64, t5054: f64, t5058: f64, t5072: f64, t5080: f64, t5088: f64, t5090: f64, t5093: f64, t5096: f64, t5099: f64, t5102: f64) -> (f64, f64, f64) {
    let t5158 = t1749 * t1179;
    let t5163 = t1757 * t1187;
    let t5180 = -0.1294625e1_f64 * t5072 + 0.258925e1_f64 * t5080 + t3503 - 0.10064166666666666667e0_f64 * t3358 - 0.10064166666666666667e0_f64 * t5044 - 0.20128333333333333333e0_f64 * t5049 + 0.60385e0_f64 * t5054 + 0.301925e0_f64 * t5058 + 0.82524375e-1_f64 * t5088 + 0.16504875e0_f64 * t5090 + t3510 - 0.5519e-1_f64 * t3415 - 0.5519e-1_f64 * t5093 - 0.27595e-1_f64 * t5096 + 0.16557e0_f64 * t5099 + 0.82785e-1_f64 * t5102;
    (t5158, t5163, t5180)
}
