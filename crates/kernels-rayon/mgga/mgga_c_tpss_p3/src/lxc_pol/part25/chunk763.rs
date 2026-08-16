//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 763/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk763(t1081: f64, t5161: f64, t2981: f64, t2988: f64, t4044: f64, t4093: f64, t5066: f64, t5070: f64, t5074: f64, t5086: f64, t5093: f64, t5099: f64, t5101: f64, t5105: f64, t5108: f64, t5111: f64) -> (f64, f64) {
    let t5162 = t5161 * t1081;
    let t5177 = -0.1294625e1_f64 * t5086 + 0.258925e1_f64 * t5093 + t2981 - 0.20128333333333333334e0_f64 * t4044 - 0.20128333333333333333e0_f64 * t5066 + 0.60385e0_f64 * t5070 + 0.301925e0_f64 * t5074 + 0.82524375e-1_f64 * t5099 + 0.16504875e0_f64 * t5101 + t2988 - 0.11038e0_f64 * t4093 - 0.27595e-1_f64 * t5105 + 0.16557e0_f64 * t5108 + 0.82785e-1_f64 * t5111;
    (t5162, t5177)
}
