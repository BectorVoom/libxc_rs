//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 964/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk964(t11159: f64, t3297: f64, t136: f64, t1113: f64, t11168: f64, t407: f64, t1102: f64, t3271: f64, t11135: f64, t11137: f64, t11139: f64, t11141: f64, t11143: f64, t11150: f64, t11156: f64, t11161: f64, t11165: f64, t11170: f64, t11174: f64) -> (f64, f64, f64, f64, f64) {
    let t11229 = t3297 * t11159;
    let t11230 = t136 * t11229;
    let t11232 = t1113 * t11168;
    let t11233 = t136 * t11232;
    let t11243 = 1.0_f64/pow_3_2(t407);
    let t11244 = t3271 * t1102;
    let t11245 = t11243 * t11244;
    let t11247 = 28.0_f64 / 27.0_f64 * t11135;
    let t11258 = -t11247 + 4.0_f64 / 9.0_f64 * t11137 + 2.0_f64 / 9.0_f64 * t11139 - 2.0_f64 / 3.0_f64 * t11141 - t11143 / 3.0_f64 + 10.0_f64 / 27.0_f64 * t11150 - 4.0_f64 / 3.0_f64 * t11156 - 2.0_f64 / 3.0_f64 * t11161 + 2.0_f64 * t11165 + 2.0_f64 * t11170 + t11174 / 3.0_f64;
    (t11230, t11233, t11244, t11245, t11258)
}
