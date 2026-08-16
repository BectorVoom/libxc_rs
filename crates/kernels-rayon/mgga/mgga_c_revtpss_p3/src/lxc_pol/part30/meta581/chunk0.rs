//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2035/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2035(t94564: f64, t9795: f64, t2018: f64, t40688: f64, t46808: f64, t7256: f64, t9784: f64, t1445: f64, t2439: f64, t25916: f64, t1358: f64, t212: f64, t26034: f64, t689: f64) -> (f64, f64, f64, f64, f64) {
    let t94565 = t94564 * t9795;
    let t94568 = t40688 * t2018 * t46808;
    let t94569 = 0.22589491248727328397e-6_f64 * t94568;
    let t94570 = t9784 * t7256;
    let t94571 = 0.14450132032386466905e-2_f64 * t94570;
    let t94580 = t2439 * t25916 * t1445;
    let t94584 = t689 * t212 * t26034 * t1358;
    (t94565, t94569, t94571, t94580, t94584)
}
