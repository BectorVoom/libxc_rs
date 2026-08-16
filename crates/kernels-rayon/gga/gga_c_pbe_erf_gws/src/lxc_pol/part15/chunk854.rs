//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 854/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk854(t625: f64, t7257: f64, t11: f64, t7205: f64, t2704: f64, t1416: f64, t2672: f64, t5022: f64, t5042: f64, t5052: f64, t5054: f64, t5056: f64, t5083: f64, t5085: f64, t5087: f64, t7233: f64, t7237: f64, t7239: f64, t7249: f64, t7252: f64, t7255: f64) -> (f64, f64, f64, f64, f64) {
    let t7258 = t625 * t7257;
    let t7259 = t11 * t7258;
    let t7261 = t625 * t7205;
    let t7262 = t2704 * t7261;
    let t7264 = t2672 * t1416;
    let t7265 = t625 * t7264;
    let t7266 = t11 * t7265;
    let t7268 = 0.71983333333333333334e-1_f64 * t7233 - 0.8888888888888888889e-2_f64 * t5022 - 0.57777777777777777777e-1_f64 * t7237 - 0.74074074074074074075e-2_f64 * t7239 - 0.23994444444444444444e-1_f64 * t5042 - 0.14814814814814814815e-1_f64 * t5052 + 0.44444444444444444445e-2_f64 * t5054 + 0.14814814814814814815e-2_f64 * t5056 - 0.31992592592592592592e-1_f64 * t5083 + 0.11997222222222222222e-1_f64 * t5085 + 0.7998148148148148148e-2_f64 * t5087 + 0.14396666666666666667e0_f64 * t7249 + 0.95977777777777777779e-1_f64 * t7252 - 0.23994444444444444445e-1_f64 * t7255 - 0.21595e0_f64 * t7259 - 0.28793333333333333334e0_f64 * t7262 + 0.71983333333333333334e-1_f64 * t7266;
    (t7259, t7262, t7264, t7266, t7268)
}
