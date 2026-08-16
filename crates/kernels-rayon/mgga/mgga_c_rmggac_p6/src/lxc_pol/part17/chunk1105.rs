//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 1105/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk1105(t40702: f64, t8571: f64, t40081: f64, t46434: f64, t7198: f64, t46438: f64, t7204: f64, t37018: f64, t42234: f64, t42239: f64, t42243: f64, t42247: f64, t42248: f64, t42250: f64, t42259: f64, t46563: f64, t48027: f64, t48029: f64, t48031: f64, t48036: f64, t48039: f64, t739: f64) -> f64 {
    let t48041 = t8571 * t40702;
    let t48043 = t8571 * t40081;
    let t48047 = t7198 * t46434;
    let t48049 = t7204 * t46438;
    let t48054 = -0.3192344991997337955e-4_f64 * t48027 + 0.3192344991997337955e-4_f64 * t48029 + 0.1064114997332445985e-4_f64 * t48031 - 0.1064114997332445985e-4_f64 * t48036 - 0.42564599893297839398e-5_f64 * t48039 - 0.25538759935978703639e-4_f64 * t48041 + 0.25538759935978703639e-4_f64 * t48043 - 0.11974241701863808564e0_f64 * t739 * t46563 - 0.40911992481368012592e-1_f64 * t48047 - 0.10227998120342003148e-1_f64 * t48049 - 0.38422568777328955684e-2_f64 * t42234 + t42239 + t42243 + t42247 + 0.72042316457491791906e-3_f64 * t42248 - 0.72042316457491791906e-3_f64 * t42250 - t42259 - t37018;
    t48054
}
