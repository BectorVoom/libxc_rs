//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 789/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk789<F: Float>(t11: F, t7248: F, t1691: F, t7212: F, t2704: F, t7097: F, t1413: F, t2678: F, t625: F, t7205: F, t1416: F, t2672: F, t5022: F, t5042: F, t5052: F, t5054: F, t5056: F, t5083: F, t5085: F, t5087: F, t7233: F, t7237: F, t7239: F) -> (F, F, F, F, F, F, F, F, F) {
    let t7249 = t11 * t7248;
    let t7251 = t1691 * t7212;
    let t7252 = t2704 * t7251;
    let t7254 = t1691 * t7097;
    let t7255 = t11 * t7254;
    let t7257 = t2678 * t1413;
    let t7258 = t625 * t7257;
    let t7259 = t11 * t7258;
    let t7261 = t625 * t7205;
    let t7262 = t2704 * t7261;
    let t7264 = t2672 * t1416;
    let t7265 = t625 * t7264;
    let t7266 = t11 * t7265;
    let t7268 = 0.71983333333333333334e-1 * t7233 - 0.8888888888888888889e-2 * t5022 - 0.57777777777777777777e-1 * t7237 - 0.74074074074074074075e-2 * t7239 - 0.23994444444444444444e-1 * t5042 - 0.14814814814814814815e-1 * t5052 + 0.44444444444444444445e-2 * t5054 + 0.14814814814814814815e-2 * t5056 - 0.31992592592592592592e-1 * t5083 + 0.11997222222222222222e-1 * t5085 + 0.7998148148148148148e-2 * t5087 + 0.14396666666666666667e0 * t7249 + 0.95977777777777777779e-1 * t7252 - 0.23994444444444444445e-1 * t7255 - 0.21595e0 * t7259 - 0.28793333333333333334e0 * t7262 + 0.71983333333333333334e-1 * t7266;
    (t7249, t7252, t7255, t7257, t7259, t7262, t7264, t7266, t7268)
}
