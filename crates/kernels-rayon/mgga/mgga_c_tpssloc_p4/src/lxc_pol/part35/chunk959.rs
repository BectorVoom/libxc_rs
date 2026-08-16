//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 959/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk959(t15979: f64, t15982: f64, t15984: f64, t15986: f64, t16164: f64, t184: f64, t20396: f64, t17: f64, t12118: f64, t12121: f64, t12123: f64, t12133: f64, t12141: f64, t9853: f64, t9859: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t20526 = 0.73245789224026180216e-3_f64 * t15979;
    let t20527 = 60.0_f64 * t15982;
    let t20528 = 36.0_f64 * t15984;
    let t20529 = 96.0_f64 * t15986;
    let t20530 = 0.35089341735807877242e1_f64 * t16164;
    let t20531 = t20396 * t184;
    let t20532 = t17 * t20531;
    let t20533 = t12118 - t12121 + t12123 + t20526 + t20527 + t20528 + t20529 + t12133 + t20530 + t9853 + t9859 - t12141 + t20532;
    (t20526, t20527, t20528, t20529, t20530, t20532, t20533)
}
