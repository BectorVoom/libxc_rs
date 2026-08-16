//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 531/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk531(t2653: f64, t336: f64, t714: f64, t1062: f64, t723: f64, t181: f64, t562: f64, t184: f64, t997: f64, t1879: f64, t1676: f64, t1027: f64, t661: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2654 = t2653 * t336;
    let t2655 = t2654 * t714;
    let t2657 = t1062 * t723;
    let t2659 = t562 * t181;
    let t2660 = t2659 * t184;
    let t2662 = 4.0_f64 / 15.0_f64 * t2660 * t997;
    let t2664 = 4.0_f64 / 15.0_f64 * t1879 * t997;
    let t2665 = 4.0_f64 / 45.0_f64 * t1676;
    let t2666 = t1027 * t661;
    (t2654, t2655, t2657, t2659, t2660, t2662, t2664, t2665, t2666)
}
