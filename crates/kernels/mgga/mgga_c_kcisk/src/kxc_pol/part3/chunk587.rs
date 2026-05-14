//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 587/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk587<F: Float>(t772: F, t2020: F, t695: F, t1060: F, t2023: F, t1775: F, t1849: F, t786: F, t3290: F, t2014: F, t3293: F, t2019: F, t785: F, t657: F, t1586: F, t5432: F, t2021: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t783 = 0.0 < t772;
    let t5491 = t2020 * t695;
    let t5492 = t1060 * t2023;
    let t5493 = t5491 * t5492;
    let t5494 = t1775 * t5493;
    let t5497 = t786 * t1849;
    let t5498 = t5497 * t3290;
    let t5499 = t1775 * t5498;
    let t5502 = t2014 * t3293;
    let t5503 = t1775 * t5502;
    let t5507 = 1.0 / t2019 / t785;
    let t5508 = t657 * t5507;
    let t5509 = t2023 * t2023;
    let t5510 = t5508 * t5509;
    let t5511 = t1586 * t5510;
    let t5515 = piecewise3(t783, t5432, -t5432);
    let t5516 = t2021 * t5515;
    (t5491, t5493, t5494, t5497, t5498, t5499, t5502, t5503, t5507, t5509, t5510, t5511, t5515, t5516)
}
