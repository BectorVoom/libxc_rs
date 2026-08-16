//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1151/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1151(t13796: f64, t14601: f64, t13859: f64, t13972: f64, t4146: f64, t3166: f64, t3990: f64, t3991: f64, t3989: f64, t3979: f64, t4150: f64, t1178: f64, t3097: f64, t371: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t14602 = t13796 * t14601;
    let t14603 = t13859 * t14602;
    let t14605 = t13972 * t4146;
    let t14608 = t3990 * t3991 * t3166;
    let t14609 = t3989 * t14608;
    let t14611 = t3979 * t4150;
    let t14614 = t371 * t1178 * t3097;
    (t14602, t14603, t14605, t14608, t14609, t14611, t14614)
}
