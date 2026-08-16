//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 870/1428 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk870(t141: f64, t5098: f64, t1145: f64, t5056: f64, t3358: f64, t3402: f64, t3414: f64, t3415: f64, t5044: f64, t5049: f64, t5054: f64, t5058: f64, t5072: f64, t5080: f64, t5088: f64, t5090: f64, t5093: f64, t5096: f64) -> (f64, f64, f64, f64) {
    let t5099 = t141 * t5098;
    let t5101 = t1145 * t5056;
    let t5102 = t141 * t5101;
    let t5104 = -0.9494625e0_f64 * t5072 + 0.1898925e1_f64 * t5080 + t3402 - 0.99655555555555555557e-1_f64 * t3358 - 0.99655555555555555557e-1_f64 * t5044 - 0.19931111111111111111e0_f64 * t5049 + 0.59793333333333333334e0_f64 * t5054 + 0.29896666666666666667e0_f64 * t5058 + 0.15358125e0_f64 * t5088 + 0.3071625e0_f64 * t5090 + t3414 - 0.54771111111111111111e-1_f64 * t3415 - 0.54771111111111111111e-1_f64 * t5093 - 0.27385555555555555556e-1_f64 * t5096 + 0.16431333333333333333e0_f64 * t5099 + 0.82156666666666666667e-1_f64 * t5102;
    (t5099, t5101, t5102, t5104)
}
