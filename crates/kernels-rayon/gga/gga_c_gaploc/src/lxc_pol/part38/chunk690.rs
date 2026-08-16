//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 690/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk690(t12804: f64, t2321: f64, t3556: f64, t882: f64, t3565: f64, t888: f64, t2268: f64, t3560: f64, t11271: f64, t3340: f64, t999: f64, t3518: f64, t894: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13303 = 0.94850022118920498664e-2_f64 * t12804;
    let t13304 = t3556 * t2321;
    let t13305 = t882 * t13304;
    let t13306 = 0.11856252764865062333e-2_f64 * t13305;
    let t13307 = t3565 * t888;
    let t13309 = 0.19918504644973304719e0_f64 * t2268 * t13307;
    let t13310 = t3560 * t2321;
    let t13311 = t882 * t13310;
    let t13312 = 0.11856252764865062333e-2_f64 * t13311;
    let t13313 = t11271 * t888;
    let t13315 = 0.85365019907028448797e-1_f64 * t2268 * t13313;
    let t13316 = t999 * t3340;
    let t13319 = t894 * t3518;
    (t13303, t13304, t13306, t13307, t13309, t13310, t13312, t13313, t13315, t13316, t13319)
}
