//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1041/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1041<F: Float>(t1965: F, t7464: F, t2597: F, t5397: F, t1676: F, t6847: F, t2386: F, t4787: F, t12090: F, t12109: F, t16353: F, t16355: F, t16358: F, t16361: F, t1975: F, t1980: F, t2605: F, t2609: F, t5368: F, t5393: F, t5401: F, t5412: F, t5416: F, t7467: F, t7490: F, t7498: F) -> (F,) {
    let t18541 = t7464 * t1965;
    let t18546 = t2597 * t5397;
    let t18553 = t6847 * t1676;
    let t18558 = t2386 * t4787;
    let t18563 = 2.0 * t18541 * t1975 + 1.0 * t7467 * t5393 + 0.32164683177870697974e2 * t18546 * t5401 + 1.0 * t12109 * t2605 + 2.0 * t5368 * t7490 + 0.11696446794910408142e1 * t18553 * t1980 + 0.58482233974552040708e0 * t7498 * t5412 + 0.17315755899375863299e2 * t18558 * t5416 + 0.58482233974552040708e0 * t12090 * t2609 - t16353 - t16355 - t16358 - t16361;
    (t18563,)
}
