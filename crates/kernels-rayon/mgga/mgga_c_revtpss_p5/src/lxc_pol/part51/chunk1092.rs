//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1092/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1092(t32322: f64, t7901: f64, t32392: f64, t7742: f64, t32394: f64, t28063: f64, t8634: f64, t28184: f64, t8568: f64, t2014: f64, t28176: f64, t32098: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t125402 = t32322 * t7901;
    let t125405 = 4.0_f64 * t32392 * t7742;
    let t125407 = 4.0_f64 * t32394 * t7742;
    let t125409 = 4.0_f64 * t8634 * t28063;
    let t125410 = t8568 * t28184;
    let t125415 = 3.0_f64 * t2014 * t32098 * t28176;
    (t125402, t125405, t125407, t125409, t125410, t125415)
}
