//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1026/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1026(t14200: f64, t783: f64, t8306: f64, t125: f64, t4701: f64, t2175: f64, t782: f64, t8279: f64, t14174: f64, t3628: f64, t3630: f64, t10600: f64, t10779: f64, t14171: f64, t14176: f64, t14181: f64, t14185: f64, t14189: f64, t14193: f64, t14197: f64, t2173: f64, t3626: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t14202 = t8306 * t14200 * t783;
    let t14205 = t125 * t4701;
    let t14207 = t2175 * t14205 * t783;
    let t14210 = t8279 * t782;
    let t14212 = t3628 * t14174 * t14210;
    let t14216 = t3628 * t14174 * t3630;
    let t14219 = t2173 * t14171 / 768.0_f64 - t3626 * t14176 / 384.0_f64 + t3626 * t14181 / 768.0_f64 - t2173 * t14185 / 3072.0_f64 + t2173 * t14189 / 768.0_f64 - t2173 * t14193 / 1536.0_f64 - t2173 * t14197 / 3072.0_f64 - 5.0_f64 / 768.0_f64 * t2173 * t14202 + t2173 * t14207 / 768.0_f64 - t10779 * t14212 / 512.0_f64 + t10600 + t3626 * t14216 / 512.0_f64;
    (t14202, t14207, t14210, t14212, t14216, t14219)
}
