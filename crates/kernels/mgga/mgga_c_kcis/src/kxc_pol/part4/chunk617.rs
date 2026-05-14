//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 617/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk617<F: Float>(t413: F, t187: F, t956: F, t1233: F, t2932: F, t2935: F, t2942: F, t2983: F, t2991: F, t2998: F, t3008: F, t3027: F, t3035: F, t3596: F, t972: F, t1260: F, t286: F) -> (F, F, F, F, F) {
    let t418 = 0.0 < t413;
    let t3600 = t187 * t956;
    let t3609 = -t2932 + t2935 - t2942 + t2983 + t2991 + t187 * t3596 + 0.19751789702565206229e-1 * t187 * t2998 - 0.11696446794910408142e1 * t3600 * t972 + 0.11696446794910408142e1 * t1233 * t3008 - 0.58482233974552040708e0 * t1233 * t3027 - 0.17315755899375863299e2 * t1233 * t3035;
    let t3611 = piecewise3(t418, t3609, -t3609);
    let t3612 = t1260 * t3611;
    let t3613 = t286 * t3612;
    (t3600, t3609, t3611, t3612, t3613)
}
