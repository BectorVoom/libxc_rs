//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1183/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1183(t12027: f64, t12030: f64, t12034: f64, t12037: f64, t12039: f64, t12040: f64, t12046: f64, t12048: f64, t11185: f64, t11188: f64, t11192: f64, t11193: f64, t11195: f64, t12043: f64, t41116: f64) -> f64 {
    let t41117 = 5.0_f64 / 8.0_f64 * t12027;
    let t41118 = 5.0_f64 / 8.0_f64 * t12030;
    let t41119 = t12034 / 2.0_f64;
    let t41120 = 5.0_f64 / 8.0_f64 * t12037;
    let t41121 = 2.0_f64 * t12039;
    let t41122 = t12040 / 2.0_f64;
    let t41123 = 3.0_f64 / 2.0_f64 * t12046;
    let t41124 = 2.0_f64 * t12048;
    let t41125 = -t11185 + t41116 - t41117 + t41118 - t41119 + t11188 - t41120 + t11192 + t41121 + t41122 + t12043 - t41123 + t11193 + t11195 + t41124;
    t41125
}
