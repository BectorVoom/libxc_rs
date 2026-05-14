//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 533/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk533<F: Float>(t1220: F, t3569: F, t1210: F, t396: F, t404: F, t3551: F, t956: F, t962: F, t265: F, t3005: F, t3006: F, t971: F, t3025: F, t3031: F, t3034: F, t1212: F, t1221: F, t1225: F, t1226: F, t2932: F, t2935: F, t2942: F, t2983: F, t2991: F, t2998: F, t3542: F, t3545: F, t3550: F, t3552: F, t405: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t3570 = t3569 * t1220;
    let t3573 = t1210 * t1210;
    let t3574 = 1.0 / t3573;
    let t3575 = t396 * t3574;
    let t3576 = t404 * t404;
    let t3577 = 1.0 / t3576;
    let t3578 = t3551 * t3577;
    let t3582 = t956 * t962;
    let t3585 = t265 * t3005;
    let t3586 = t3006 * t971;
    let t3589 = t3025 * t971;
    let t3592 = t265 * t3031;
    let t3593 = t3006 * t3034;
    let t3596 = -0.3109e-1 * t3542 * t405 + 2.0 * t3545 * t1221 - 2.0 * t3550 * t3552 + 1.0 * t1212 * t3570 + 0.32164683177870697974e2 * t3575 * t3578 + t2932 - t2935 + t2942 - t2983 - t2991 - 0.19751789702565206229e-1 * t2998 + 0.11696446794910408142e1 * t3582 * t1226 - 0.11696446794910408142e1 * t3585 * t3586 + 0.58482233974552040708e0 * t1225 * t3589 + 0.17315755899375863299e2 * t3592 * t3593;
    (t3570, t3573, t3574, t3575, t3576, t3577, t3578, t3582, t3585, t3586, t3589, t3592, t3593, t3596)
}
