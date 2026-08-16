//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 756/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk756(t1703: f64, t395: f64, t1693: f64, t1639: f64, t56: f64, t1672: f64, t662: f64, t211: f64, t1794: f64, t582: f64, t648: f64, t618: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5085 = t395 * t1703;
    let t5087 = t395 * t1693;
    let t5089 = t56 * t1639;
    let t5102 = t1672 * t662;
    let t5103 = t211 * t5102;
    let t5105 = t582 * t1794;
    let t5106 = t211 * t5105;
    let t5108 = t648 * t648;
    let t5109 = 1.0_f64 / t5108;
    let t5116 = t1672 * t618;
    (t5085, t5087, t5089, t5103, t5106, t5109, t5116)
}
