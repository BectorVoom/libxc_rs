//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 619/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk619<F: Float>(t13305: F, t3565: F, t888: F, t2268: F, t2321: F, t3560: F, t882: F, t11271: F, t3518: F, t894: F, t3531: F, t12831: F, t11288: F, t921: F, t3366: F, t8045: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t13306 = 0.11856252764865062333e-2 * t13305;
    let t13307 = t3565 * t888;
    let t13309 = 0.19918504644973304719e0 * t2268 * t13307;
    let t13310 = t3560 * t2321;
    let t13311 = t882 * t13310;
    let t13312 = 0.11856252764865062333e-2 * t13311;
    let t13313 = t11271 * t888;
    let t13315 = 0.85365019907028448797e-1 * t2268 * t13313;
    let t13319 = t894 * t3518;
    let t13321 = 0.28455006635676149599e-1 * t2268 * t13319;
    let t13327 = t894 * t3531;
    let t13329 = 0.28455006635676149599e-1 * t2268 * t13327;
    let t13330 = 0.142275033178380748e-1 * t12831;
    let t13334 = t11288 * t921;
    let t13338 = 4.0 * t8045 * t3366;
    (t13306, t13307, t13309, t13310, t13312, t13313, t13315, t13319, t13321, t13327, t13329, t13330, t13334, t13338)
}
