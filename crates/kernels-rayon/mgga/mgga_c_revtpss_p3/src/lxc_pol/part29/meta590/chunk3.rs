//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1961/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1961(t3923: f64, t8085: f64, t136: f64, t2457: f64, t8094: f64, t94589: f64, t26072: f64, t28845: f64, t28840: f64, t686: f64, t72: f64, t25895: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t102185 = t8085 * t3923;
    let t102204 = t8094 * t136 * t2457;
    let t102205 = t94589 * t102204;
    let t102213 = 0.14456046980341999104e-1_f64 * t26072 * t28845;
    let t102215 = t28840 * t72 * t686;
    let t102217 = 0.28912093960683998208e-1_f64 * t25895 * t102215;
    (t102185, t102204, t102205, t102213, t102215, t102217)
}
