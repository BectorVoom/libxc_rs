//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 904/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk904(t31772: f64, t4364: f64, t886: f64, t31767: f64, t1032: f64, t8471: f64, t867: f64, t786: f64, t233: f64, t72: f64, t686: f64, t7063: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t31774 = t4364 * t31772 * t886;
    let t31775 = t31767 * t31774;
    let t31777 = t8471 * t1032;
    let t31778 = t31777 * t867;
    let t31779 = t786 * t31778;
    let t31780 = t233 * t72;
    let t31781 = t31780 * t686;
    let t31783 = 0.14456046980341999104e-1_f64 * t31779 * t31781;
    let t31784 = t7063 * t31778;
    (t31774, t31775, t31777, t31778, t31779, t31780, t31781, t31783, t31784)
}
