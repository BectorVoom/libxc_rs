//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 855/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk855(t1398: f64, t1419: f64, t4086: f64, t543: f64, t2782: f64, t4056: f64, t555: f64, t9990: f64, t1432: f64, t2470: f64, t4107: f64, t1433: f64, t9288: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10079 = t4086 * t1419 * t1398 * t543;
    let t10080 = t2782 * t10079;
    let t10082 = t555 * t4056;
    let t10084 = t4086 * t10082 * t543;
    let t10085 = t2782 * t10084;
    let t10090 = t9990 * t555;
    let t10098 = t1432 * t4107 * t2470;
    let t10102 = 0.30356481678079769392e-1_f64 * t1432 * t1433 * t9288;
    (t10080, t10082, t10085, t10090, t10098, t10102)
}
