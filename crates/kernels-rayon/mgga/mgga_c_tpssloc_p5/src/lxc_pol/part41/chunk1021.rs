//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 1021/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk1021(t1788: f64, t2221: f64, t225: f64, t5213: f64, t5211: f64, t12248: f64, t68: f64, t544: f64, t5230: f64, t12189: f64, t1804: f64, t5194: f64, t782: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t15984 = t2221 * t1788;
    let t16022 = t5213 * t225;
    let t16030 = t5211 * t225;
    let t16046 = t68 * t12248;
    let t16047 = t544 * t16046;
    let t16060 = t5230 * t68;
    let t16078 = t12189 * t1804;
    let t16081 = t782 * t5194;
    (t15984, t16022, t16030, t16047, t16060, t16078, t16081)
}
