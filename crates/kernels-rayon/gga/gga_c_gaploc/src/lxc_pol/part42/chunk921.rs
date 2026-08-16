//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 921/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk921(t2299: f64, t3529: f64, t1415: f64, t1646: f64, t46094: f64, t6716: f64, t6717: f64, t42189: f64, t10526: f64, t20471: f64, t2487: f64, t46115: f64, t6711: f64) -> (f64, f64, f64, f64, f64) {
    let t46550 = t2299 * t3529;
    let t46553 = 0.35750489951850426669e0_f64 * t1415 * t46550 * t1646;
    let t46559 = 0.62115540045351614476e2_f64 * t6716 * t6717 * t46094;
    let t46564 = 0.17875244975925213335e0_f64 * t42189;
    let t46567 = 0.21450293971110256001e1_f64 * t20471 * t10526 * t46094;
    let t46570 = 0.87421871174939309262e2_f64 * t2487 * t6711 * t46115;
    (t46553, t46559, t46564, t46567, t46570)
}
