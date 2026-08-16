//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 721/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk721(t1401: f64, t7264: f64, t1405: f64, t2019: f64, t545: f64, t64: f64) -> (f64, f64, f64) {
    let t7265 = t7264 * t1401;
    let t7267 = t2019 * t1405;
    let t7268 = 0.20007875121765877254e-2_f64 * t7267;
    let t7269 = t545 * t64;
    (t7265, t7268, t7269)
}
