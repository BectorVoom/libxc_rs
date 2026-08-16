//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3106/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3106(t1161: f64, t1169: f64, t17089: f64, t1757: f64, t20521: f64, t20526: f64, t24331: f64, t24363: f64, t24366: f64, t3447: f64, t45080: f64, t45197: f64, t5120: f64, t5181: f64, t58317: f64, t6506: f64, t6535: f64, t69354: f64, t81128: f64, t81130: f64, t81132: f64, t81134: f64, t81136: f64, t81138: f64, t81678: f64, t81691: f64, t81705: f64, t81717: f64, t81729: f64, t81740: f64, t81754: f64, t81766: f64) -> f64 {
    let t81781 = -t81128 - t81130 - t81132 - t81134 - t81136 + t81138 + 3.0_f64 * t5120 * t20521 + 0.96491876992155210402e2_f64 * t58317 * t6506 - 0.19298375398431042081e3_f64 * t45197 * t24331 + 1.0_f64 * t3447 * t24363 + 1.0_f64 * t1161 * (t81678 + t81691 + t81705 + t81717 + t81729 + t81740 + t81754 + t81766) * t1169 + 0.2069040516770936012e4_f64 * t45080 * t24366 + 0.17544670867903938621e1_f64 * t69354 * t1757 + 0.17544670867903938621e1_f64 * t20526 * t5181 + 0.17544670867903938621e1_f64 * t17089 * t6535;
    t81781
}
