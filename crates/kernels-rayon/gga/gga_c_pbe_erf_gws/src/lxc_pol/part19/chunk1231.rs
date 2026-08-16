//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1231/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1231(t1114: f64, t51916: f64, t50935: f64, t1112: f64, t2306: f64, t3074: f64, t833: f64, t837: f64, t13972: f64, t14799: f64, t1176: f64, t21518: f64, t367: f64) -> (f64, f64, f64, f64, f64) {
    let t53566 = t1114 * t51916;
    let t53571 = t1114 * t50935;
    let t53577 = t3074 * t2306 * t1112 * t837 * t833;
    let t53583 = t13972 * t14799;
    let t53592 = t1176 * t367 * t21518;
    (t53566, t53571, t53577, t53583, t53592)
}
