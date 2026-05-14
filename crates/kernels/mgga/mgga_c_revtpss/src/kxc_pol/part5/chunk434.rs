//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 434/1286 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk434<F: Float>(t1455: F, t3: F, t571: F, t117: F, t670: F, t572: F, t573: F, t578: F, t582: F, t586: F, t590: F, t594: F, t598: F, t4: F, t604: F) -> (F, F, F, F, F, F, F) {
    let t1456 = t3 * t1455;
    let t1458 = t3 * t571;
    let t1459 = param_d * t1455;
    let t1461 = t117 * t670;
    let t1464 = t1459 * t573 + 3.0 * t1461 * t572;
    let t1466 = -t578 - t582 - t586 - t590 - t594 - t598;
    let t1468 = -t4 - t604;
    (t1456, t1458, t1459, t1461, t1464, t1466, t1468)
}
