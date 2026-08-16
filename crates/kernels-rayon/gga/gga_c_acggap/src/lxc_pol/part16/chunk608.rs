//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 608/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk608(t1165: f64, t1532: f64, t5720: f64, t1753: f64, t322: f64, t1181: f64, t1163: f64, t1748: f64, t3194: f64, t301: f64, t513: f64, t944: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5722 = t1165 * t1532 * t5720;
    let t5725 = t1753 * t322;
    let t5726 = t1532 * t5725;
    let t5727 = t1181 * t5726;
    let t5728 = t1163 * t5727;
    let t5730 = t1748 * t322;
    let t5732 = t1165 * t1532 * t5730;
    let t5733 = t3194 * t5732;
    let t5735 = t1748 * t301;
    let t5737 = t1165 * t1532 * t5735;
    let t5740 = t944 * t513;
    (t5722, t5727, t5728, t5732, t5733, t5737, t5740)
}
