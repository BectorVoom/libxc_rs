//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 705/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk705(t305: f64, t9812: f64, t9706: f64, t9710: f64, t9714: f64, t9721: f64, t9724: f64, t9727: f64, t1704: f64, t699: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10185 = t305 * t9812;
    let t10241 = 0.40911992481368012596e-1_f64 * t9706;
    let t10242 = 0.16364796992547205038e0_f64 * t9710;
    let t10243 = 0.5454932330849068346e-1_f64 * t9714;
    let t10247 = 0.72042316457491791901e-3_f64 * t9721;
    let t10248 = 0.1440846329149835838e-2_f64 * t9724;
    let t10249 = 0.1440846329149835838e-2_f64 * t9727;
    let t10252 = t699 * t1704;
    (t10185, t10241, t10242, t10243, t10247, t10248, t10249, t10252)
}
