//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 698/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk698<F: Float>(t1974: F, t7489: F, t2604: F, t5400: F, t1973: F, t1676: F, t2386: F, t1683: F, t2609: F, t1685: F, t6874: F, t2394: F, t4790: F, t1966: F, t1975: F, t1979: F, t1980: F, t2605: F, t5368: F, t5373: F, t5398: F, t5405: F, t5408: F, t5415: F, t6801: F, t6804: F, t6806: F, t6809: F, t6837: F, t6841: F, t6848: F, t7464: F, t7467: F, t7472: F, t764: F) -> (F, F, F, F, F, F, F, F, F) {
    let t7490 = t7489 * t1974;
    let t7493 = t2604 * t5400;
    let t7494 = t7493 * t1973;
    let t7498 = t2386 * t1676;
    let t7503 = t2609 * t1683;
    let t7506 = t6874 * t1685;
    let t7509 = t2394 * t4790;
    let t7510 = t7509 * t1683;
    let t7513 = -0.3109e-1 * t7464 * t764 + 1.0 * t7467 * t1975 + 1.0 * t5368 * t2605 - 2.0 * t5373 * t7472 + 1.0 * t1966 * t7490 + 0.32164683177870697974e2 * t5398 * t7494 + t6801 - t6804 - t6806 + t6809 - t6837 - t6841 - 0.19751789702565206229e-1 * t6848 + 0.58482233974552040708e0 * t7498 * t1980 + 0.58482233974552040708e0 * t5405 * t2609 - 0.11696446794910408142e1 * t5408 * t7503 + 0.58482233974552040708e0 * t1979 * t7506 + 0.17315755899375863299e2 * t5415 * t7510;
    (t7490, t7493, t7494, t7498, t7503, t7506, t7509, t7510, t7513)
}
