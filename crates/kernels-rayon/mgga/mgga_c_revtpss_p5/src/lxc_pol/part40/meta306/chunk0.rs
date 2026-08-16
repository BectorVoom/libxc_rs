//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1077/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1077(t11200: f64, t378: f64, t3043: f64, t3042: f64, t993: f64, t1071: f64, t989: f64, t3056: f64, t988: f64, t1031: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11201 = t11200 * t378;
    let t11210 = t3043 * t378;
    let t11213 = t3042 * t993;
    let t11214 = t11213 * t378;
    let t11220 = t989 * t1071;
    let t11223 = t988 * t3056;
    let t11224 = t11223 * t378;
    let t11238 = t1031 * t1031;
    let t11239 = 1.0_f64 / t11238;
    (t11201, t11210, t11213, t11214, t11220, t11223, t11224, t11239)
}
