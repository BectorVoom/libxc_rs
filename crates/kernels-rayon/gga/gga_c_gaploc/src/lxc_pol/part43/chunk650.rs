//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 650/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk650(t12000: f64, t569: f64, t568: f64, t3701: f64, t524: f64, t189: f64, t188: f64, t600: f64, t1628: f64, t3709: f64, t10479: f64, t10484: f64, t10501: f64, t10503: f64, t10506: f64, t10508: f64, t10510: f64, t10512: f64, t1580: f64, t1641: f64, t193: f64, t3702: f64, t3710: f64, t3715: f64, t541: f64, t574: f64, t597: f64) -> (f64, f64) {
    let t12109 = t569 * t12000;
    let t12110 = t568 * t12109;
    let t12113 = t524 * t3701;
    let t12116 = t189 * t12000;
    let t12117 = t188 * t12116;
    let t12124 = t600 * t12000;
    let t12125 = t568 * t12124;
    let t12128 = t1628 * t3709;
    let t12131 = -0.23005755572352449806e1_f64 * t1641 * t3710 - 0.23005755572352449806e1_f64 * t574 * t12110 + 0.35750489951850426669e0_f64 * t12113 * t193 + 0.35750489951850426669e0_f64 * t12117 * t193 + 0.23833659967900284446e0_f64 * t3702 * t541 + 0.23005755572352449806e1_f64 * t1580 * t3715 + 0.23005755572352449806e1_f64 * t597 * t12125 - 0.30674340763136599741e1_f64 * t574 * t12128 + t10479 + t10484 - t10501 - t10503 + t10506 + t10508 + t10510 + t10512;
    (t12116, t12131)
}
