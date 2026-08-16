//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2195/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2195(t2248: f64, t77: f64, t7705: f64, t10301: f64, t1470: f64, t2247: f64, t4181: f64, t4187: f64, t10309: f64, t13388: f64, t76: f64, t13269: f64, t607: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t101234 = t77 * t7705 * t2248;
    let t101237 = t10301 * t1470;
    let t101240 = t2247 * t4181;
    let t101243 = t2247 * t4187;
    let t101252 = t10309 * t1470;
    let t101303 = t76 * t13388;
    let t101323 = t13269 * t607;
    (t101234, t101237, t101240, t101243, t101252, t101303, t101323)
}
