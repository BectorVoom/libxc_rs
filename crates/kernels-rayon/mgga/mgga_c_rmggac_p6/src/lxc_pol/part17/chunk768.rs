//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 768/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk768(t36156: f64, t35875: f64, t793: f64, t35924: f64, t797: f64, t262: f64, t3899: f64, t661: f64, t851: f64, t854: f64, t305: f64, t655: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t36157 = 0.30289299735990067054e-2_f64 * t36156;
    let t36166 = t793 * t35875;
    let t36168 = t797 * t35924;
    let t36172 = t262 * t3899;
    let t36173 = t661 * t36172;
    let t36174 = 0.68992293843088486071e-3_f64 * t36173;
    let t36188 = t851 * t35875;
    let t36190 = t854 * t35924;
    let t36200 = t305 * t3899;
    let t36201 = 0.22765842247987981715e0_f64 * t36200;
    let t36204 = t655 * t36172;
    (t36157, t36166, t36168, t36174, t36188, t36190, t36201, t36204)
}
