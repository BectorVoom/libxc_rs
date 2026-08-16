//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2081/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2081(t6897: f64, t6907: f64, t90544: f64, t26203: f64, t6883: f64, t7700: f64, t80645: f64, t225: f64, t26219: f64, t214: f64, t5318: f64, t26378: f64, t6914: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t90701 = t6897 * t90544 * t6907;
    let t90702 = 0.82246703342411321824e-2_f64 * t90701;
    let t90707 = t6883 * t26203;
    let t90708 = 0.38381794893125283518e-1_f64 * t90707;
    let t90723 = t6897 * t80645 * t7700;
    let t90724 = 0.82246703342411321824e-2_f64 * t90723;
    let t90732 = t26219 * t225;
    let t90739 = t214 * t5318;
    let t90749 = t6914 * t26378;
    (t90702, t90708, t90724, t90732, t90739, t90749)
}
