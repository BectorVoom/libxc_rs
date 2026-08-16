//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 350/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk350(t1133: f64, t1141: f64, t1142: f64, t1143: f64, t220: f64, t468: f64, t1139: f64, t1134: f64, t1136: f64, t473: f64, t475: f64) -> (f64, f64, f64, f64) {
    let t1148 = t1133 * t220 * t468 + t1141 * t1142 * t1143;
    let t1149 = t1139 * t1148;
    let t1151 = t1134 * t473 - t1136 * t1149;
    let t1153 = 1.0_f64 / t475;
    (t1148, t1149, t1151, t1153)
}
