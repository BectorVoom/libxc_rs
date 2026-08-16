//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 773/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk773(t3335: f64, t389: f64, t1077: f64, t992: f64, t1031: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11108 = 1.0_f64 / t3335 / t389;
    let t11119 = t1077 * t1077;
    let t11120 = 1.0_f64 / t11119;
    let t11198 = t992 * t992;
    let t11199 = 1.0_f64 / t11198;
    let t11238 = t1031 * t1031;
    let t11239 = 1.0_f64 / t11238;
    (t11108, t11119, t11120, t11198, t11199, t11238, t11239)
}
