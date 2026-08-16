//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 918/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk918(t42071: f64, t46121: f64, t544: f64, t40166: f64, t10525: f64, t2365: f64, t35959: f64, t13420: f64, t4614: f64, t574: f64, t37326: f64, t895: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t46498 = 0.51123901271894332902e0_f64 * t42071;
    let t46499 = t544 * t46121;
    let t46500 = t46499 * t40166;
    let t46501 = 0.17875244975925213335e0_f64 * t46500;
    let t46503 = t10525 * t2365 * t35959;
    let t46504 = 0.89376224879626066674e-1_f64 * t46503;
    let t46507 = 0.12269736305254639897e2_f64 * t574 * t4614 * t13420;
    let t46516 = 0.23833659967900284446e0_f64 * t895 * t37326;
    (t46498, t46499, t46501, t46504, t46507, t46516)
}
