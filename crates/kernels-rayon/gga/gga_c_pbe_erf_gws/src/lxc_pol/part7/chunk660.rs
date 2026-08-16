//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 660/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk660(t187: f64, t190: f64, t5044: f64, t25: f64, t4941: f64, t4943: f64, t4945: f64, t4947: f64, t4954: f64, t4969: f64, t4974: f64, t4978: f64, t5233: f64, t5236: f64) -> f64 {
    let t5241 = 0.29629629629629629629e-1_f64 * t190 * t5044 * t187;
    let t5245 = -0.47988888888888888888e-1_f64 * t4941 + 0.35991666666666666666e-1_f64 * t4947 + 0.23994444444444444444e-1_f64 * t4943 - 0.39990740740740740742e-1_f64 * t4954 - 0.35991666666666666667e-1_f64 * t4978 - 0.39999999999999999999e-1_f64 * t25 * t5233 + 0.39999999999999999999e-1_f64 * t25 * t5236 - t5241 - 0.21595e0_f64 * t4969 + 0.21595e0_f64 * t4974 - 0.71983333333333333333e-1_f64 * t4945;
    t5245
}
