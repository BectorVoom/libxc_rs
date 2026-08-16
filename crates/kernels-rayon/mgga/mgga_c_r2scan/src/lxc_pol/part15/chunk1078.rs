//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1078/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1078(t10856: f64, t5174: f64, t2111: f64, t2164: f64, t22766: f64, t20450: f64, t2215: f64, t10710: f64, t10768: f64, t20437: f64, t10734: f64, t571: f64, t572: f64) -> (f64, f64, f64, f64, f64) {
    let t37998 = t10856 * t5174;
    let t38001 = t2111 * t22766 * t2164;
    let t38002 = 0.1590300183910403919e-2_f64 * t38001;
    let t38003 = t20450 * t2215;
    let t38028 = t10768 * t10710 * t20437;
    let t38031 = t571 * t572 * t10734;
    (t37998, t38002, t38003, t38028, t38031)
}
