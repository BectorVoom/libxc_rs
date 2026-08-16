//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 857/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk857(t657: f64, t7205: f64, t1714: f64, t7093: f64, t7257: f64, t190: f64, t25: f64, t2718: f64, t5044: f64, t5047: f64, t5082: f64, t7269: f64, t7272: f64, t7274: f64, t7279: f64, t7280: f64, t7285: f64, t7288: f64, t7290: f64, t7291: f64, t7294: f64, t7297: f64, t7300: f64) -> f64 {
    let t7303 = t657 * t7205;
    let t7306 = t1714 * t7093;
    let t7309 = t657 * t7257;
    let t7312 = -0.15996296296296296296e-1_f64 * t7269 - 0.26393888888888888889e0_f64 * t7272 + 0.13333333333333333333e-1_f64 * t190 * t5044 * t7274 - t5047 - t5082 + t7279 - 0.47988888888888888889e-1_f64 * t7280 - 0.39990740740740740742e-1_f64 * t7285 - t7288 + t7290 - 0.22222222222222222222e-2_f64 * t25 * t7291 - 0.29629629629629629629e-2_f64 * t25 * t7294 + 0.88888888888888888887e-2_f64 * t2718 * t7297 + 0.13333333333333333333e-1_f64 * t25 * t7300 - 0.53333333333333333332e-1_f64 * t2718 * t7303 + 0.13333333333333333333e-1_f64 * t25 * t7306 - 0.39999999999999999999e-1_f64 * t25 * t7309;
    t7312
}
