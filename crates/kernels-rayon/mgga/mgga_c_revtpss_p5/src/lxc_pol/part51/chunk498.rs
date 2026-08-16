//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 498/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk498(t136: f64, t1413: f64, t1353: f64, t221: f64, t3978: f64, t247: f64, t2682: f64, t550: f64, t548: f64, t1408: f64, t820: f64, t843: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3979 = t1413 * t136;
    let t3981 = t3979 * t221 * t1353;
    let t3982 = t3978 * t3981;
    let t3985 = t2682 * t550 * t247;
    let t3987 = 0.56688979511669985553e-2_f64 * t548 * t3985;
    let t3989 = t820 * t1408 * t843;
    (t3979, t3981, t3982, t3985, t3987, t3989)
}
