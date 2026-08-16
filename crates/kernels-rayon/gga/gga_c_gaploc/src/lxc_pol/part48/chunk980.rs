//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 980/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk980(t2299: f64, t3529: f64, t1415: f64, t1646: f64, t46094: f64, t6716: f64, t6717: f64, t10480: f64, t10532: f64, t10533: f64, t40372: f64, t42146: f64, t42170: f64, t42183: f64, t42187: f64, t46080: f64, t46084: f64, t46521: f64, t46526: f64, t46529: f64, t46535: f64, t46537: f64, t46539: f64, t46542: f64, t46544: f64, t46547: f64, t46549: f64, t8411: f64) -> f64 {
    let t46550 = t2299 * t3529;
    let t46553 = 0.35750489951850426669e0_f64 * t1415 * t46550 * t1646;
    let t46559 = 0.62115540045351614476e2_f64 * t6716 * t6717 * t46094;
    let t46563 = -t46521 + 0.21450293971110256002e1_f64 * t8411 * t10480 - t46526 - t46529 + 0.13803453343411469884e2_f64 * t6716 * t6717 * t46084 + t46535 + t46537 - 0.1022478025437886658e1_f64 * t42146 + t46539 - t46542 + t46544 + t46547 + t46549 - t46553 + 0.55213813373645879536e2_f64 * t10532 * t10533 * t46080 + t46559 - 0.63904876589867916126e-1_f64 * t40372 - t42170 + 0.38342925953920749677e1_f64 * t42183 - 0.85206502119823888171e0_f64 * t42187;
    t46563
}
