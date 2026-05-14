//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1284/1286 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1284<F: Float>(t21660: F, t22531: F, t3: F, t1913: F, t1921: F, t571: F, t6951: F, t5883: F, t670: F, t4292: F, t5801: F, t116: F, t5920: F, t117: F, t21881: F, t1459: F, t1461: F, t1916: F, t1918: F, t572: F, t573: F, t5795: F, t5802: F, t5805: F, t6941: F, t6945: F, t6948: F) -> (F, F, F, F) {
    let t22532 = t21660 + t22531;
    let t22533 = t3 * t22532;
    let t22536 = t1913 * t1921;
    let t22542 = t571 * t6951;
    let t22544 = param_d * t22532;
    let t22556 = t670 * t5883;
    let t22559 = t5801 * t4292;
    let t22564 = t116 * t5920;
    let t22565 = t22564 * t670;
    let t22568 = t117 * t21881;
    let t22571 = 6.0 * t1459 * t6945 + 3.0 * t1459 * t6948 + 3.0 * t1461 * t6941 + 12.0 * t1916 * t5802 + 6.0 * t1916 * t5805 + 6.0 * t1918 * t5795 + t22544 * t573 + 6.0 * t22556 * t572 + 12.0 * t22559 * t572 + 6.0 * t22565 * t572 + 3.0 * t22568 * t572;
    (t22533, t22536, t22542, t22571)
}
