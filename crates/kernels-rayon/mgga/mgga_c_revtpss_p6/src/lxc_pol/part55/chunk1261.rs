//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1261/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1261(t125930: f64, t121356: f64, t122512: f64, t125925: f64, t125928: f64, t128844: f64, t128846: f64, t128850: f64, t128852: f64, t128854: f64, t128856: f64, t3140: f64, t5710: f64, t8477: f64, t8709: f64) -> f64 {
    let t128859 = 0.17354086964223805049e-2_f64 * t125930;
    let t128860 = 0.57119737665102352616e0_f64 * t8477 * t5710 * t3140 * t8709 - 0.14279934416275588154e-1_f64 * t128844 + 0.25389723392137995738e-1_f64 * t128846 - 0.69416347856895220196e-2_f64 * t121356 - 0.14279934416275588154e-1_f64 * t128850 - 0.14456046980341999104e-1_f64 * t128852 + 0.25702851531048074406e-1_f64 * t128854 + t122512 - t128856 + 0.26447628533477078895e-3_f64 * t125925 - 0.3718732920905101082e-3_f64 * t125928 + t128859;
    t128860
}
