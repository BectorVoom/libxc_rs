//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 905/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk905(t10: f64, t10089: f64, t10090: f64, t10094: f64, t10096: f64, t10097: f64, t10130: f64, t496: f64, t5784: f64, t5810: f64, t8148: f64, t8149: f64, t8158: f64, t8160: f64) -> f64 {
    let t10132 = t10089 + t10090 + t8148 - 0.195872e1_f64 * t8149 + t8158 - 0.97936e0_f64 * t8160 - 2.0_f64 / 9.0_f64 * t5784 + t10094 - 0.97935999999999999999e0_f64 * t5810 - t10096 + 3.0_f64 / 2.0_f64 * t496 * t10 * t10097 + t10130;
    t10132
}
