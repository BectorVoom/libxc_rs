//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2084/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2084(t26392: f64, t80670: f64, t22705: f64, t26422: f64, t81228: f64, t22704: f64, t26466: f64, t26461: f64, t26433: f64, t6883: f64, t22716: f64, t7741: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t90837 = t80670 * t26392;
    let t90844 = t81228 * t22705 * t26422;
    let t90845 = 0.16449340668482264365e-1_f64 * t90844;
    let t90859 = t22704 * t22705 * t26466;
    let t90860 = 0.82246703342411321824e-2_f64 * t90859;
    let t90864 = t22704 * t22705 * t26461;
    let t90865 = 0.82246703342411321824e-2_f64 * t90864;
    let t90866 = t6883 * t26433;
    let t90867 = 0.38381794893125283518e-1_f64 * t90866;
    let t90868 = t22716 * t7741;
    (t90837, t90845, t90860, t90865, t90867, t90868)
}
