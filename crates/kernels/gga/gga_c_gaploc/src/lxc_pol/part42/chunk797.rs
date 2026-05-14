//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 797/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk797<F: Float>(t46536: F, t42156: F, t10525: F, t10526: F, t46254: F, t37654: F, t901: F, t34506: F, t34507: F, t46362: F, t11402: F, t9285: F, t2299: F, t3529: F, t1415: F, t1646: F) -> (F, F, F, F, F, F, F) {
    let t46537 = 0.29792074959875355558e-1 * t46536;
    let t46539 = 0.3575048995185042667e0 * t42156;
    let t46542 = 0.21450293971110256001e1 * t10525 * t10526 * t46254;
    let t46543 = t37654 * t901;
    let t46544 = 0.14896037479937677779e-1 * t46543;
    let t46547 = 0.85801175884441024004e1 * t34506 * t34507 * t46362;
    let t46549 = 0.35750489951850426669e0 * t9285 * t11402;
    let t46550 = t2299 * t3529;
    let t46553 = 0.35750489951850426669e0 * t1415 * t46550 * t1646;
    (t46537, t46539, t46542, t46544, t46547, t46549, t46553)
}
