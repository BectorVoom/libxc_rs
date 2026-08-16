//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1021/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1021(t76122: f64, t76127: f64, t76130: f64, t76132: f64, t71836: f64, t1469: f64, t34976: f64, t39851: f64, t699: f64, t34975: f64, t9145: f64, t16503: f64, t35039: f64, t8420: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t78501 = 0.44903406381989282115e-1_f64 * t76122;
    let t78502 = 0.30487649791575028312e-3_f64 * t76127;
    let t78503 = 0.72042316457491791901e-3_f64 * t76130;
    let t78504 = 0.85129199786595678799e-5_f64 * t76132;
    let t78514 = 0.39914139006212695213e-1_f64 * t71836;
    let t78517 = t39851 * t34976 * t699 * t1469;
    let t78518 = 0.85129199786595678796e-5_f64 * t78517;
    let t78521 = t34975 * t34976 * t699 * t9145;
    let t78522 = 0.53205749866622299248e-5_f64 * t78521;
    let t78525 = t16503 * t35039 * t699 * t8420;
    (t78501, t78502, t78503, t78504, t78514, t78518, t78522, t78525)
}
