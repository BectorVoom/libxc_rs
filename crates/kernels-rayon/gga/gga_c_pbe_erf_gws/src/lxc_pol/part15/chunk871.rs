//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 871/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk871(t1879: f64, t2800: f64, t219: f64, t641: f64, t1697: f64, t422: f64, t954: f64, t617: f64, t5211: f64, t1639: f64, t1642: f64, t5219: f64, t995: f64) -> (f64, f64, f64, f64, f64) {
    let t7482 = 8.0_f64 / 15.0_f64 * t1879 * t2800;
    let t7483 = t641 * t219;
    let t7484 = t7483 * t1697;
    let t7485 = t954 * t422;
    let t7486 = t7485 * t617;
    let t7487 = t7484 * t7486;
    let t7489 = 32.0_f64 / 45.0_f64 * t5211 * t7487;
    let t7490 = t1639 * t219;
    let t7491 = t7490 * t1642;
    let t7492 = t7491 * t7486;
    let t7494 = 16.0_f64 / 27.0_f64 * t5211 * t7492;
    let t7495 = t5219 * t995;
    (t7482, t7483, t7489, t7494, t7495)
}
