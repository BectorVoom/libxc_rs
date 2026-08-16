//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3121/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3121(t1168: f64, t1187: f64, t1189: f64, t12429: f64, t12553: f64, t16997: f64, t17023: f64, t17026: f64, t17032: f64, t20606: f64, t20609: f64, t20668: f64, t24431: f64, t435: f64, t5125: f64, t5147: f64, t58345: f64, t6503: f64, t6534: f64, t69376: f64, t69488: f64, t81649: f64, t81653: f64, t81656: f64, t81660: f64, t82050: f64, t82060: f64, t82093: f64, t82111: f64) -> f64 {
    let t82115 = t81649 - t81653 - t81656 - t81660 + 0.5848223622634646207e0_f64 * t82050 * t1189 + 3.0_f64 * t17026 * t6503 + 0.30762056574649219973e4_f64 * t12553 * t6534 * t16997 * t1187 - t82060 - 6.0_f64 * t69488 * t5125 + 0.96491876992155210402e2_f64 * t69376 * t5147 + 18.0_f64 * t17032 * t20606 - 12.0_f64 * t17023 * t20609 - 24.0_f64 * t12429 * t24431 * t1168 + 0.10526802520742363173e2_f64 * t58345 * t20668 - 0.310907e-1_f64 * (t82093 + t82111) * t435;
    t82115
}
