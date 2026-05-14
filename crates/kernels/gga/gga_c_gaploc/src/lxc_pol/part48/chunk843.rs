//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 843/861 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk843<F: Float>(t37654: F, t901: F, t34506: F, t34507: F, t46362: F, t11402: F, t9285: F, t2299: F, t3529: F, t1415: F, t1646: F, t46094: F, t6716: F, t6717: F, t10480: F, t10532: F, t10533: F, t40372: F, t42146: F, t42170: F, t42183: F, t42187: F, t46080: F, t46084: F, t46521: F, t46526: F, t46529: F, t46535: F, t46537: F, t46539: F, t46542: F, t8411: F) -> (F,) {
    let t46543 = t37654 * t901;
    let t46544 = 0.14896037479937677779e-1 * t46543;
    let t46547 = 0.85801175884441024004e1 * t34506 * t34507 * t46362;
    let t46549 = 0.35750489951850426669e0 * t9285 * t11402;
    let t46550 = t2299 * t3529;
    let t46553 = 0.35750489951850426669e0 * t1415 * t46550 * t1646;
    let t46559 = 0.62115540045351614476e2 * t6716 * t6717 * t46094;
    let t46563 = -t46521 + 0.21450293971110256002e1 * t8411 * t10480 - t46526 - t46529 + 0.13803453343411469884e2 * t6716 * t6717 * t46084 + t46535 + t46537 - 0.1022478025437886658e1 * t42146 + t46539 - t46542 + t46544 + t46547 + t46549 - t46553 + 0.55213813373645879536e2 * t10532 * t10533 * t46080 + t46559 - 0.63904876589867916126e-1 * t40372 - t42170 + 0.38342925953920749677e1 * t42183 - 0.85206502119823888171e0 * t42187;
    (t46563,)
}
