//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 394/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk394<F: Float>(t147: F, t184: F, t2304: F, t21: F, t363: F, t649: F, t1580: F, t185: F, t2236: F, t2240: F, t2301: F, t5: F, t620: F, t623: F, t650: F, t342: F, t630: F, t657: F) -> (F, F, F, F, F) {
    let t148 = 10000000.0 <= t147;
    let t2305 = t2304 * t184;
    let t2306 = t2305 * t21;
    let t2309 = t649 * t363;
    let t2316 = piecewise3(t148, 0.0, t5 * t2236 * t21 / 4.0 + t2240 * t650 / 2.0 + t5 * t620 * t363 / 2.0 + t623 * t2301 / 4.0 + t623 * t2306 / 4.0 + t623 * t2309 / 2.0 + t5 * t185 * t1580 / 4.0);
    let t2319 = t342 * t630 * t657 / 12.0;
    (t2305, t2306, t2309, t2316, t2319)
}
