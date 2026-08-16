//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1136/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1136(t13972: f64, t4146: f64, t3166: f64, t3990: f64, t3991: f64, t3989: f64, t3979: f64, t4150: f64, t1178: f64, t3097: f64, t371: f64, t3983: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t14605 = t13972 * t4146;
    let t14608 = t3990 * t3991 * t3166;
    let t14609 = t3989 * t14608;
    let t14611 = t3979 * t4150;
    let t14614 = t371 * t1178 * t3097;
    let t14615 = t3983 * t14614;
    (t14605, t14608, t14609, t14611, t14614, t14615)
}
