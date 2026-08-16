//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 392/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk392(t1096: f64, t1121: f64, t1124: f64, t1129: f64, t1138: f64, t1144: f64, t1148: f64, t1157: f64, t300: f64, t436: f64, t440: f64) -> (f64, f64, f64) {
    let t1161 = t300 * (-0.310907e-1_f64 * t1124 * t436 + 1.0_f64 * t1129 * t1138 + t1096 - t1121 - 0.19751673498613801407e-1_f64 * t1144 + 0.5848223622634646207e0_f64 * t1148 * t1157);
    let t1163 = 0.19751673498613801407e-1_f64 * t300 * t1144;
    let t1164 = t300 * t440;
    (t1161, t1163, t1164)
}
