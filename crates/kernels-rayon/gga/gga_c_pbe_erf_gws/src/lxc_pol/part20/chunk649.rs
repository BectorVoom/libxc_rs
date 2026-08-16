//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 649/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk649(t125: f64, t1463: f64, t1467: f64, t1471: f64, t1475: f64, t1482: f64, t1486: f64, t1499: f64, t169: f64, t2937: f64, t2939: f64, t299: f64, t301: f64, t3373: f64, t3574: f64) -> f64 {
    let t3577 = -t1463 + t1467 + t1471 - t1475 - t1482 + t1486 - t1499 - 0.23948468020509218188e-1_f64 * t2937 + 0.20267214298646782767e-1_f64 * t169 * t299 * t3373 * t301 + t3574 * t125 + 0.39914113367515363646e-1_f64 * t2939;
    t3577
}
