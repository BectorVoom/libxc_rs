//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1853/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1853(t3678: f64, t7613: f64, t3685: f64, t7607: f64, t3596: f64, t3598: f64, t3594: f64, t1238: f64, t26817: f64, t26821: f64, t26822: f64, t26824: f64, t26827: f64, t3606: f64, t3663: f64, t3674: f64, t3689: f64, t3694: f64, t3701: f64, t484: f64, sigma2: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t26832 = t7613 * t3678;
    let t26836 = t7607 * t3685;
    let t26842 = t3596 * sigma2;
    let t26843 = t26842 * t3598;
    let t26844 = t3594 * t26843;
    let t26847 = 0.42874018118069736972e-3_f64 * t26817 * t484 - t26821 + 0.57165357490759649296e-3_f64 * t26822 + 0.85748036236139473944e-3_f64 * t26824 * t3674 - 0.85748036236139473944e-3_f64 * t26827 * t1238 - 0.42874018118069736972e-3_f64 * t7613 * t3663 - 0.57165357490759649296e-3_f64 * t26832 + t7607 * t3701 / 216.0_f64 - t26836 / 432.0_f64 - t7607 * t3689 / 288.0_f64 - t7607 * t3694 / 144.0_f64 + 0.85748036236139473944e-3_f64 * t26844 * t3606;
    (t26832, t26836, t26842, t26843, t26844, t26847)
}
