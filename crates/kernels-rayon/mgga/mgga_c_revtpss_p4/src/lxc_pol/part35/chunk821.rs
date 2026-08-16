//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 821/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk821(t125: f64, t6016: f64, t2741: f64, t5980: f64, t5966: f64, t2652: f64, t5993: f64, t6030: f64, t10858: f64, t6024: f64, t6019: f64, t10811: f64, t6037: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t18444 = t125 * t6016;
    let t18459 = t2741 * t5980;
    let t18469 = t125 * t5966;
    let t18475 = t2652 * t5993;
    let t18485 = t2652 * t6030;
    let t18487 = t10858 * t6024;
    let t18491 = t2741 * t6019;
    let t18518 = t10811 * t6037;
    (t18444, t18459, t18469, t18475, t18485, t18487, t18491, t18518)
}
