//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 796/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk796<F: Float>(t11: F, t7394: F, t571: F, t7355: F, t2704: F, t7350: F, t1014: F, t1251: F, t2781: F, t7236: F, t5248: F, t5256: F, t5258: F, t5260: F, t7374: F, t7376: F, t7379: F, t7380: F, t7383: F, t7386: F, t7389: F, t7392: F) -> (F, F, F, F) {
    let t7395 = t11 * t7394;
    let t7397 = t571 * t7355;
    let t7398 = t2704 * t7397;
    let t7400 = t571 * t7350;
    let t7401 = t11 * t7400;
    let t7407 = t1251 * t1014;
    let t7409 = t7236 * t2781;
    let t7411 = -0.15996296296296296296e-1 * t7374 + 0.26393888888888888889e0 * t7376 + t7379 - 0.47988888888888888889e-1 * t7380 - 0.39990740740740740742e-1 * t7383 + 0.14396666666666666667e0 * t7386 - 0.95977777777777777779e-1 * t7389 - 0.23994444444444444445e-1 * t7392 - 0.21595e0 * t7395 + 0.28793333333333333334e0 * t7398 + 0.71983333333333333334e-1 * t7401 - 0.8888888888888888889e-2 * t5248 - 0.14814814814814814815e-1 * t5256 + 0.44444444444444444445e-2 * t5258 + 0.14814814814814814815e-2 * t5260 - 0.74074074074074074075e-2 * t7407 + 0.57777777777777777777e-1 * t7409;
    (t7395, t7398, t7401, t7411)
}
