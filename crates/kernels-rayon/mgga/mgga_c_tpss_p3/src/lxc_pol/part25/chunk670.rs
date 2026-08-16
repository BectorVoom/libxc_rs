//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 670/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk670(t4245: f64, t450: f64, t1112: f64, t242: f64, t1098: f64, t1103: f64, t1111: f64, t3027: f64, t3029: f64, t3052: f64, t3067: f64, t4210: f64, t4212: f64, t4217: f64, t4220: f64, t4224: f64, t4228: f64, t4234: f64, t4239: f64, t4242: f64) -> (f64, f64, f64) {
    let t4246 = t4245 * t450;
    let t4247 = t1112 * t4246;
    let t4248 = t242 * t4247;
    let t4251 = -t4210 / 108.0_f64 + t4212 * t1103 / 108.0_f64 - t3027 - t3029 / 864.0_f64 - t4217 / 864.0_f64 + t1098 * t4220 / 216.0_f64 - t1098 * t4224 / 144.0_f64 - t1098 * t4228 / 288.0_f64 + t3052 * t4234 / 1536.0_f64 + t4239 / 4608.0_f64 - t3067 * t4242 / 4608.0_f64 + t1111 * t4248 / 3072.0_f64;
    (t4246, t4248, t4251)
}
