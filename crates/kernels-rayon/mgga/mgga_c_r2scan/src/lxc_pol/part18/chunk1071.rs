//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1071/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1071(t2111: f64, t2164: f64, t22766: f64, t20450: f64, t2215: f64, t10734: f64, t571: f64, t572: f64, t22948: f64, t37945: f64, t254: f64, t259: f64, t277: f64, t37449: f64) -> (f64, f64, f64, f64, f64) {
    let t38001 = t2111 * t22766 * t2164;
    let t38002 = 0.1590300183910403919e-2_f64 * t38001;
    let t38003 = t20450 * t2215;
    let t38031 = t571 * t572 * t10734;
    let t38033 = t38031 * t37945 * t22948;
    let t38054 = t254 * t259 * t37449 * t277;
    (t38002, t38003, t38031, t38033, t38054)
}
