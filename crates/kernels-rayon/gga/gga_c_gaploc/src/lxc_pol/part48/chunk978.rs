//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 978/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk978(t13402: f64, t2487: f64, t6985: f64, t34400: f64, t34401: f64, t46362: f64, t10525: f64, t10526: f64, t46115: f64, t46103: f64, t6716: f64, t6717: f64) -> (f64, f64, f64, f64) {
    let t46520 = t2487 * t6985 * t13402;
    let t46521 = 0.25561950635947166451e0_f64 * t46520;
    let t46526 = 0.13803453343411469884e3_f64 * t34400 * t34401 * t46362;
    let t46529 = 0.42900587942220512002e1_f64 * t10525 * t10526 * t46115;
    let t46535 = 0.69017266717057349418e1_f64 * t6716 * t6717 * t46103;
    (t46521, t46526, t46529, t46535)
}
