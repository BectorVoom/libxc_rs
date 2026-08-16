//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 1081/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk1081(t71836: f64, t1469: f64, t34976: f64, t39851: f64, t699: f64, t34975: f64, t9145: f64, t16503: f64, t35039: f64, t8420: f64, t76504: f64, t1664: f64, t3207: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t78514 = 0.39914139006212695213e-1_f64 * t71836;
    let t78517 = t39851 * t34976 * t699 * t1469;
    let t78518 = 0.85129199786595678796e-5_f64 * t78517;
    let t78521 = t34975 * t34976 * t699 * t9145;
    let t78522 = 0.53205749866622299248e-5_f64 * t78521;
    let t78525 = t16503 * t35039 * t699 * t8420;
    let t78526 = 0.42564599893297839398e-5_f64 * t78525;
    let t78528 = 0.1702583995731913576e-4_f64 * t76504;
    let t78529 = t1664 * t3207;
    (t78514, t78518, t78522, t78526, t78528, t78529)
}
