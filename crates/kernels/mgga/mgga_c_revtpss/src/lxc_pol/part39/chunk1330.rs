//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1330/1348 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1330<F: Float>(t31303: F, t31326: F, t3: F, t2178: F, t670: F, t1518: F, t31117: F, t4292: F, t8295: F, t116: F, t8362: F, t117: F, t31292: F, t1459: F, t1461: F, t1916: F, t1918: F, t2187: F, t2189: F, t572: F, t573: F, t5795: F, t5802: F, t5805: F, t8289: F, t8296: F, t8299: F, t8377: F, t8383: F, t8386: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t31328 = 2.0 * t31303 + 2.0 * t31326;
    let t31329 = t3 * t31328;
    let t31340 = param_d * t31328;
    let t31358 = t670 * t2178;
    let t31359 = t31358 * t1518;
    let t31362 = t31117 * t1518;
    let t31365 = t8295 * t4292;
    let t31370 = t116 * t8362;
    let t31371 = t31370 * t670;
    let t31374 = t117 * t31292;
    let t31377 = 6.0 * t1459 * t8383 + 3.0 * t1459 * t8386 + 3.0 * t1461 * t8377 + 6.0 * t1916 * t8296 + 3.0 * t1916 * t8299 + 3.0 * t1918 * t8289 + 6.0 * t2187 * t5802 + 3.0 * t2187 * t5805 + 3.0 * t2189 * t5795 + t31340 * t573 + 6.0 * t31359 * t572 + 6.0 * t31362 * t572 + 6.0 * t31365 * t572 + 6.0 * t31371 * t572 + 3.0 * t31374 * t572;
    (t31328, t31329, t31340, t31358, t31359, t31362, t31365, t31370, t31371, t31374, t31377)
}
