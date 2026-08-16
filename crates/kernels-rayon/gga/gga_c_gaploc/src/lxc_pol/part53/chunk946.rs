//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 946/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk946(t46867: f64, t550: f64, t1358: f64, t1365: f64, t13749: f64, t158: f64, t123: f64, t488: f64, t13740: f64, t2312: f64, t2325: f64, t38413: f64, t882: f64, t883: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t46868 = t550 * t46867;
    let t46871 = 0.31616674039640166221e-2_f64 * t1358 * t1365 * t46868;
    let t46873 = t158 * t13749;
    let t46877 = 0.31616674039640166221e-2_f64 * t1358 * t46873 * t123 * t488;
    let t46878 = t2312 * t13740;
    let t46884 = t882 * t2325 * t883 * t38413;
    (t46868, t46871, t46873, t46877, t46878, t46884)
}
