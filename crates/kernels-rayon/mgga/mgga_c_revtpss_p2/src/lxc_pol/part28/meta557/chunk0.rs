//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2011/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2011(t10690: f64, t1945: f64, t9646: f64, t10674: f64, t807: f64, t7030: f64, t9789: f64, t2453: f64, t2783: f64, t64: f64, t10761: f64, t9784: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t93007 = t9646 * t1945 * t10690;
    let t93008 = 0.4016411544023718989e-6_f64 * t93007;
    let t93010 = t807 * t1945 * t10674;
    let t93012 = t9789 * t7030;
    let t93013 = 0.22589491248727328397e-6_f64 * t93012;
    let t93015 = t2453 * t2783 * t64;
    let t93016 = t93015 * t10761;
    let t93020 = t9784 * t7030;
    (t93008, t93010, t93013, t93015, t93016, t93020)
}
