//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2060/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2060(t94570: f64, t1445: f64, t2439: f64, t25916: f64, t1358: f64, t212: f64, t26034: f64, t689: f64, t25877: f64, t94390: f64, t94385: f64, t9675: f64) -> (f64, f64, f64, f64, f64) {
    let t94571 = 0.14450132032386466905e-2_f64 * t94570;
    let t94580 = t2439 * t25916 * t1445;
    let t94584 = t689 * t212 * t26034 * t1358;
    let t94589 = t94390 * t25877;
    let t94590 = t94385 * t9675;
    (t94571, t94580, t94584, t94589, t94590)
}
