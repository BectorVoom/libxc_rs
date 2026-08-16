//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1139/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1139(t3189: f64, t3974: f64, t3990: f64, t14637: f64, t2409: f64, t8590: f64, t3965: f64, t14113: f64, t4142: f64, t1114: f64, t13791: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t14639 = t3990 * t3974 * t3189;
    let t14640 = t14637 * t14639;
    let t14648 = t2409 * t8590;
    let t14649 = t3965 * t14648;
    let t14655 = t14113 * t4142;
    let t14657 = t1114 * t13791;
    (t14639, t14640, t14648, t14649, t14655, t14657)
}
