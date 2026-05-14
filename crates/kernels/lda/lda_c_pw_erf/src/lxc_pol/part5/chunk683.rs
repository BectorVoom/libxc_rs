//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 683/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk683<F: Float>(t11: F, t6651: F, t557: F, t6446: F, t1953: F, t2420: F, t325: F, t1349: F, t6366: F, t6361: F, t558: F, t6005: F, t3627: F, t4013: F, t4657: F, t4659: F, t4662: F, t4663: F, t6638: F, t6641: F, t6644: F, t6647: F, t6649: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t6652 = t11 * t6651;
    let t6654 = t557 * t6446;
    let t6655 = t1953 * t6654;
    let t6657 = t325 * t2420;
    let t6659 = t1349 * t6366;
    let t6660 = t11 * t6659;
    let t6662 = t557 * t6361;
    let t6663 = t11 * t6662;
    let t6665 = t558 * t6005;
    let t6666 = t557 * t6665;
    let t6667 = t11 * t6666;
    let t6669 = t4013 + 0.0008396296296296296 * t3627 + 0.0016792592592592592 * t4657 - 0.0008396296296296296 * t4659 + t4662 + 0.002518888888888889 * t4663 - 0.0004198148148148148 * t6638 + 0.002099074074074074 * t6641 - 0.007556666666666666 * t6644 - 0.005037777777777778 * t6647 + 0.0012594444444444445 * t6649 + 0.011335 * t6652 + 0.015113333333333333 * t6655 - 0.0006297222222222223 * t6657 + 0.0012594444444444445 * t6660 - 0.003778333333333333 * t6663 + 0.0018891666666666666 * t6667;
    (t6652, t6654, t6655, t6657, t6659, t6660, t6662, t6663, t6665, t6666, t6667, t6669)
}
