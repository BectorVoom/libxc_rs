//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 686/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk686<F: Float>(t12804: F, t2321: F, t3556: F, t882: F, t3565: F, t888: F, t2268: F, t3560: F, t11271: F, t3518: F, t894: F, t3531: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t13303 = F::cast_from(0.94850022118920498664e-2_f64) * t12804;
    let t13304 = t3556 * t2321;
    let t13305 = t882 * t13304;
    let t13306 = F::cast_from(0.11856252764865062333e-2_f64) * t13305;
    let t13307 = t3565 * t888;
    let t13309 = F::cast_from(0.19918504644973304719e0_f64) * t2268 * t13307;
    let t13310 = t3560 * t2321;
    let t13311 = t882 * t13310;
    let t13312 = F::cast_from(0.11856252764865062333e-2_f64) * t13311;
    let t13313 = t11271 * t888;
    let t13315 = F::cast_from(0.85365019907028448797e-1_f64) * t2268 * t13313;
    let t13319 = t894 * t3518;
    let t13321 = F::cast_from(0.28455006635676149599e-1_f64) * t2268 * t13319;
    let t13327 = t894 * t3531;
    (t13303, t13304, t13306, t13307, t13309, t13310, t13312, t13313, t13315, t13319, t13321, t13327)
}
