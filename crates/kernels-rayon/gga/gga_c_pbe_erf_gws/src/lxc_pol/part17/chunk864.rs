//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 864/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk864(t2781: f64, t7236: f64, t5248: f64, t5256: f64, t5258: f64, t5260: f64, t7374: f64, t7376: f64, t7379: f64, t7380: f64, t7383: f64, t7386: f64, t7389: f64, t7392: f64, t7395: f64, t7398: f64, t7401: f64, t7407: f64) -> f64 {
    let t7409 = t7236 * t2781;
    let t7411 = -0.15996296296296296296e-1_f64 * t7374 + 0.26393888888888888889e0_f64 * t7376 + t7379 - 0.47988888888888888889e-1_f64 * t7380 - 0.39990740740740740742e-1_f64 * t7383 + 0.14396666666666666667e0_f64 * t7386 - 0.95977777777777777779e-1_f64 * t7389 - 0.23994444444444444445e-1_f64 * t7392 - 0.21595e0_f64 * t7395 + 0.28793333333333333334e0_f64 * t7398 + 0.71983333333333333334e-1_f64 * t7401 - 0.8888888888888888889e-2_f64 * t5248 - 0.14814814814814814815e-1_f64 * t5256 + 0.44444444444444444445e-2_f64 * t5258 + 0.14814814814814814815e-2_f64 * t5260 - 0.74074074074074074075e-2_f64 * t7407 + 0.57777777777777777777e-1_f64 * t7409;
    t7411
}
