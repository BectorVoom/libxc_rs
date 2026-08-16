//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 630/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk630(t5355: f64, t5524: f64, t2038: f64, t2041: f64, t2040: f64, t801: f64, t798: f64, t2049: f64, t5275: f64, t5279: f64, t5281: f64, t5287: f64, t5292: f64, t5296: f64, t5300: f64, t5304: f64, t5308: f64, t5311: f64, t5313: f64, t5318: f64, t5324: f64, t5328: f64, t5333: f64, t5337: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5525 = t5355 + t5524;
    let t5527 = t2038 * t2041;
    let t5531 = 1.0_f64 / t2040 / t801;
    let t5532 = t798 * t5531;
    let t5533 = t2049 * t2049;
    let t5552 = 0.9375e-1_f64 * t5275 - 0.1875e0_f64 * t5279 + 0.125e0_f64 * t5281 + 0.1875e0_f64 * t5287 - 0.125e0_f64 * t5292 - 0.9375e-1_f64 * t5296 - 0.20833333333333333333e-1_f64 * t5300 + 0.625e-1_f64 * t5304 - 0.101171875e-1_f64 * t5308 + 0.20234375e-1_f64 * t5311 - 0.26979166666666666666e-1_f64 * t5313 - 0.20234375e-1_f64 * t5318 + 0.26979166666666666666e-1_f64 * t5324 + 0.101171875e-1_f64 * t5328 - 0.44965277777777777777e-2_f64 * t5333 - 0.13489583333333333333e-1_f64 * t5337;
    (t5525, t5527, t5531, t5532, t5533, t5552)
}
