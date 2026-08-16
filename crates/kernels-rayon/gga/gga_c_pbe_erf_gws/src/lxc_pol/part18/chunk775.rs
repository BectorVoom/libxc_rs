//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 775/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk775(t5434: f64, t712: f64, t1903: f64, t708: f64, t703: f64, t713: f64, t155: f64, t641: f64, t644: f64, t639: f64, t1639: f64, t9: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5436 = 0.2e-20_f64 * t712 * t5434;
    let t5437 = t708 * t1903;
    let t5441 = t703 * t713;
    let t5443 = 0.13506172839506172839e-1_f64 * t712 * t5441;
    let t5463 = t155 * t641;
    let t5464 = t5463 * t644;
    let t5465 = t639 * t5464;
    let t5480 = t9 * t1639;
    (t5436, t5437, t5443, t5463, t5465, t5480)
}
