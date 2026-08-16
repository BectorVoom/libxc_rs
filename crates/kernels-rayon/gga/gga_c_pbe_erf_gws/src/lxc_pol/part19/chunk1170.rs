//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1170/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1170(t14043: f64, t14048: f64, t14529: f64, t14531: f64, t14533: f64, t14536: f64, t14539: f64, t14542: f64, t14544: f64, t14549: f64, t15070: f64, t14554: f64) -> (f64, f64) {
    let t15071 = -t14529 / 384.0_f64 - t14531 / 96.0_f64 - t14533 / 24.0_f64 - t14536 / 24.0_f64 - t14539 / 48.0_f64 + t14043 - t14542 / 24.0_f64 + t14544 / 384.0_f64 + t14048 + t14549 / 8.0_f64 - t15070;
    let t15072 = 7.0_f64 / 144.0_f64 * t14554;
    (t15071, t15072)
}
