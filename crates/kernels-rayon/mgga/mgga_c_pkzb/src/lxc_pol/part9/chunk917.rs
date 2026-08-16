//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 917/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk917(t2648: f64, t6966: f64, t1733: f64, t5244: f64, t5297: f64, t5299: f64, t5301: f64, t5315: f64, t5379: f64, t5382: f64, t5385: f64, t5405: f64, t6941: f64, t6946: f64, t6958: f64, t6963: f64) -> f64 {
    let t6968 = 0.20007875121765877254e-2_f64 * t6966 * t2648;
    let t6969 = 0.17149607247227894789e-2_f64 * t1733 * t6941 - 0.34299214494455789578e-2_f64 * t5244 * t6946 - 0.45351183609335988442e-1_f64 * t5297 + 0.40015750243531754508e-2_f64 * t5299 - 0.20007875121765877254e-1_f64 * t5301 + 0.10003937560882938627e-2_f64 * t5315 + 0.10003937560882938627e-2_f64 * t5379 - 0.20007875121765877254e-2_f64 * t5382 - 0.11337795902333997111e-1_f64 * t5385 - t5405 + 0.17149607247227894789e-2_f64 * t1733 * t6958 + 0.85748036236139473944e-3_f64 * t1733 * t6963 + t6968;
    t6969
}
