//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 676/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk676(t3788: f64, t860: f64, t1109: f64, t5: f64, t337: f64, t2121: f64, t3116: f64, t3128: f64, t3180: f64, t3703: f64, t858: f64, t2210: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3790 = t3788 * t860 / 96.0_f64;
    let t3791 = t5 * t1109;
    let t3792 = t337 * t3791;
    let t3793 = t2121 * t3792;
    let t3795 = t3116 * t3793 / 96.0_f64;
    let t3797 = t3128 * t3180 / 24.0_f64;
    let t3798 = t858 * t3703;
    let t3799 = t2210 * t3798;
    (t3790, t3791, t3792, t3793, t3795, t3797, t3799)
}
