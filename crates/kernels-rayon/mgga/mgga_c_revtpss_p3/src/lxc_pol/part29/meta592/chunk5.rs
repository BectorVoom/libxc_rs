//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1972/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1972(t102315: f64, t25899: f64, t2439: f64, t8099: f64, t94391: f64, t102234: f64, t3916: f64, t25895: f64, t2097: f64, t9990: f64, t102115: f64, t7289: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t102378 = t25899 * t102315;
    let t102385 = t8099 * t2439;
    let t102386 = t94391 * t102385;
    let t102394 = t102234 * t3916;
    let t102396 = 0.28912093960683998208e-1_f64 * t25895 * t102394;
    let t102397 = t9990 * t2097;
    let t102404 = 0.25702851531048074406e-1_f64 * t7289 * t102115;
    (t102378, t102385, t102386, t102394, t102396, t102397, t102404)
}
