//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 772/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk772(t1444: f64, t676: f64, t123: f64, t3915: f64, t1363: f64, t2470: f64, t1362: f64, t1398: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3916 = t676 * t1444;
    let t3917 = t123 * t3916;
    let t3918 = t3915 * t3917;
    let t3920 = t1363 * t2470;
    let t3922 = 0.13009920719177044025e-1_f64 * t1362 * t3920;
    let t3923 = t1398 * t1398;
    (t3916, t3917, t3918, t3920, t3922, t3923)
}
