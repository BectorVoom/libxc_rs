//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 1044/1228 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk1044(t17271: f64, t894: f64, t901: f64, t17157: f64, t2826: f64, t136: f64, t5717: f64, t699: f64, t5720: f64, t5723: f64, t17187: f64, t908: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t17272 = t894 * t17271;
    let t17274 = t901 * t17271;
    let t17279 = t2826 * t17157;
    let t17280 = t136 * t17279;
    let t17286 = t699 * t5717;
    let t17288 = t699 * t5720;
    let t17290 = t699 * t5723;
    let t17292 = t908 * t17187;
    (t17272, t17274, t17280, t17286, t17288, t17290, t17292)
}
