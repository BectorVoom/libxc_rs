//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1057/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1057(t3792: f64, t6183: f64, t3116: f64, t11844: f64, t11846: f64, t11849: f64, t11852: f64, t11854: f64, t11857: f64, t11862: f64, t11863: f64, t11864: f64, t11867: f64, t2253: f64, t6456: f64, t9539: f64) -> (f64, f64) {
    let t11868 = t6183 * t3792;
    let t11869 = t3116 * t11868;
    let t11870 = 7.0_f64 / 288.0_f64 * t11869;
    let t11871 = t11844 - 119.0_f64 / 6912.0_f64 * t6456 - 7.0_f64 / 768.0_f64 * t11846 - t2253 * t11849 / 768.0_f64 + 7.0_f64 / 576.0_f64 * t11852 - t2253 * t11854 / 768.0_f64 + t9539 - 7.0_f64 / 288.0_f64 * t11857 + t11862 - t11863 - 7.0_f64 / 1152.0_f64 * t11864 - t11867 + t11870;
    (t11870, t11871)
}
