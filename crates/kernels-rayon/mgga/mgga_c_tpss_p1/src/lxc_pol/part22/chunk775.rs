//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 775/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk775(t1098: f64, t4216: f64, t3032: f64, t926: f64, t4047: f64, t1100: f64, t4052: f64, t1101: f64, t3431: f64, t1561: f64, t461: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4217 = t1098 * t4216;
    let t4219 = t926 * t3032;
    let t4220 = t4219 * t4047;
    let t4223 = t926 * t1100;
    let t4224 = t4223 * t4052;
    let t4227 = t1101 * t3431;
    let t4228 = t926 * t4227;
    let t4231 = t461 * t1561;
    (t4217, t4219, t4220, t4223, t4224, t4227, t4228, t4231)
}
