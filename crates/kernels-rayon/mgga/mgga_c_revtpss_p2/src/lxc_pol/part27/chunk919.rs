//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 919/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk919(t10493: f64, t11082: f64, t11092: f64, t11093: f64, t1100: f64, t3333: f64, t3335: f64, t389: f64, t2918: f64, t936: f64, t2874: f64, t2926: f64, t934: f64) -> (f64, f64, f64, f64, f64) {
    let t11095 = t10493 + t11082 + t11092 + t11093;
    let t11105 = t3333 * t1100;
    let t11108 = 1.0_f64 / t3335 / t389;
    let t11112 = t936 * t2918;
    let t11114 = 6.0_f64 * t2874 * t11112;
    let t11116 = t2918 * t2926 * t934;
    (t11095, t11105, t11108, t11114, t11116)
}
