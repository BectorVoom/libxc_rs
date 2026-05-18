//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 920/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk920<F: Float>(t13393: F, t18067: F, t42156: F, t10525: F, t10526: F, t46254: F, t37654: F, t901: F, t34506: F, t34507: F, t46362: F, t11402: F, t9285: F) -> (F, F, F, F, F, F) {
    let t46536 = t18067 * t13393;
    let t46537 = F::new(0.29792074959875355558e-1) * t46536;
    let t46539 = F::new(0.3575048995185042667e0) * t42156;
    let t46542 = F::new(0.21450293971110256001e1) * t10525 * t10526 * t46254;
    let t46543 = t37654 * t901;
    let t46544 = F::new(0.14896037479937677779e-1) * t46543;
    let t46547 = F::new(0.85801175884441024004e1) * t34506 * t34507 * t46362;
    let t46549 = F::new(0.35750489951850426669e0) * t9285 * t11402;
    (t46537, t46539, t46542, t46544, t46547, t46549)
}
