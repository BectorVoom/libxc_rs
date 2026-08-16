//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 976/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk976(t42066: f64, t37965: f64, t895: f64, t42071: f64, t46121: f64, t544: f64, t40166: f64, t10525: f64, t2365: f64, t35959: f64, t13420: f64, t4614: f64, t574: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t46491 = 0.23005755572352449806e1_f64 * t42066;
    let t46497 = 0.35750489951850426669e0_f64 * t895 * t37965;
    let t46498 = 0.51123901271894332902e0_f64 * t42071;
    let t46499 = t544 * t46121;
    let t46500 = t46499 * t40166;
    let t46501 = 0.17875244975925213335e0_f64 * t46500;
    let t46503 = t10525 * t2365 * t35959;
    let t46504 = 0.89376224879626066674e-1_f64 * t46503;
    let t46507 = 0.12269736305254639897e2_f64 * t574 * t4614 * t13420;
    (t46491, t46497, t46498, t46499, t46501, t46504, t46507)
}
