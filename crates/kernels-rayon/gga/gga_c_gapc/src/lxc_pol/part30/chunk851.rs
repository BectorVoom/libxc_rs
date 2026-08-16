//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 851/1331 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk851(t10158: f64, t996: f64, t3218: f64, t1560: f64, t315: f64, t2160: f64, t2165: f64, t3244: f64, t126: f64, t2190: f64, t284: f64, t3201: f64, t763: f64) -> (f64, f64, f64, f64, f64) {
    let t10159 = t996 * t10158;
    let t10160 = t10159 * t3218;
    let t10162 = t1560 * t315;
    let t10163 = t2160 * t10162;
    let t10165 = t2165 * t3244;
    let t10167 = t126 * t2190;
    let t10168 = t284 * t10167;
    let t10172 = t763 * t3201;
    (t10160, t10163, t10165, t10168, t10172)
}
