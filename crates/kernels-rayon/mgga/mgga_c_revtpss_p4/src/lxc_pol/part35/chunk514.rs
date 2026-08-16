//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 514/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk514(t1032: f64, t1770: f64, t1246: f64, t1263: f64, t1774: f64, t1038: f64, t1802: f64, t1244: f64, t1241: f64, t1121: f64, t3362: f64, t3617: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5273 = t1770 * t1032;
    let t5274 = t5273 * t1246;
    let t5277 = t1263 * t1774;
    let t5291 = t1802 * t1038;
    let t5292 = t1244 * t5291;
    let t5293 = t1241 * t5292;
    let t5296 = t1263 * t1121;
    let t5302 = t3617 * t3362;
    (t5273, t5274, t5277, t5292, t5293, t5296, t5302)
}
