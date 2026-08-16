//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 920/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk920(t13393: f64, t18067: f64, t42156: f64, t10525: f64, t10526: f64, t46254: f64, t37654: f64, t901: f64, t34506: f64, t34507: f64, t46362: f64, t11402: f64, t9285: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t46536 = t18067 * t13393;
    let t46537 = 0.29792074959875355558e-1_f64 * t46536;
    let t46539 = 0.3575048995185042667e0_f64 * t42156;
    let t46542 = 0.21450293971110256001e1_f64 * t10525 * t10526 * t46254;
    let t46543 = t37654 * t901;
    let t46544 = 0.14896037479937677779e-1_f64 * t46543;
    let t46547 = 0.85801175884441024004e1_f64 * t34506 * t34507 * t46362;
    let t46549 = 0.35750489951850426669e0_f64 * t9285 * t11402;
    (t46537, t46539, t46542, t46544, t46547, t46549)
}
