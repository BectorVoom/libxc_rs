//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 819/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk819(t2571: f64, t4934: f64, t1620: f64, t219: f64, t2591: f64, t649: f64, t1639: f64, t331: f64, t1621: f64, t1791: f64, t21: f64, t5589: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7191 = t4934 * t2571;
    let t7193 = 32.0_f64 / 135.0_f64 * t1620 * t7191;
    let t7194 = t2591 * t219;
    let t7199 = t2591 * t649;
    let t7209 = t331 * t1639;
    let t7210 = t7209 * t219;
    let t7216 = t1621 * t1791;
    let t7236 = t21 * t5589;
    (t7193, t7194, t7199, t7210, t7216, t7236)
}
