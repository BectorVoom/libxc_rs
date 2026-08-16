//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 878/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk878(t342: f64, t4980: f64, t3154: f64, t3302: f64, t1043: f64, t4893: f64, t1071: f64, t1089: f64, t1668: f64, t378: f64, t4866: f64, t3316: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4981 = t342 * t4980;
    let t4982 = t3302 * t3154;
    let t4983 = t4982 * t1043;
    let t4984 = t4893 * t4983;
    let t4988 = t1071 * t1668 * t1089;
    let t4992 = t378 * t4866 * t1089;
    let t4995 = t3316 * t378;
    (t4981, t4982, t4983, t4984, t4988, t4992, t4995)
}
