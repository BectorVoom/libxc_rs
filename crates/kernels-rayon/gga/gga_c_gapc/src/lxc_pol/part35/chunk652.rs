//! GGA_C_GAPC lxc pol — lxc_pol part 35 (v4rho2sigma2_14) CSE chunk 652/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part35_v4rho2sigma2_14_chunk652(t1552: f64, t1672: f64, t1338: f64, t1971: f64, t198: f64, t1037: f64, t457: f64, t505: f64, t1689: f64, t567: f64, t147: f64, t1601: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5075 = t1672 * t1552;
    let t5079 = t1672 * t1338;
    let t5116 = t1971 * t198;
    let t5117 = t1037 * t457;
    let t5121 = t1037 * t505;
    let t5126 = t1689 * t567;
    let t5144 = t1601 * t147;
    (t5075, t5079, t5116, t5117, t5121, t5126, t5144)
}
