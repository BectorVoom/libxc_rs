//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2541/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2541(t52035: f64, t11199: f64, t1646: f64, t378: f64, t11120: f64, t1695: f64, t11200: f64, t1678: f64, t3056: f64, t4742: f64, t379: f64, t51973: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t52955 = 0.22222222222222222222e-1_f64 * t52035;
    let t53014 = t1646 * t11199;
    let t53015 = t53014 * t378;
    let t53108 = t11120 * t1695;
    let t53160 = t11200 * t1678;
    let t53166 = t4742 * t3056;
    let t53167 = t53166 * t378;
    let t53174 = t11200 * t379;
    let t53243 = 0.19755555555555555556e-1_f64 * t51973;
    (t52955, t53014, t53015, t53108, t53160, t53166, t53167, t53174, t53243)
}
