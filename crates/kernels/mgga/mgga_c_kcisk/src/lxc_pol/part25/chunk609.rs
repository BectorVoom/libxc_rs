//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 609/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk609<F: Float>(t5491: F, t5492: F, t1775: F, t1849: F, t786: F, t3290: F, t2014: F, t3293: F, t2019: F, t785: F) -> (F, F, F, F, F, F, F, F) {
    let t5493 = t5491 * t5492;
    let t5494 = t1775 * t5493;
    let t5497 = t786 * t1849;
    let t5498 = t5497 * t3290;
    let t5499 = t1775 * t5498;
    let t5502 = t2014 * t3293;
    let t5503 = t1775 * t5502;
    let t5507 = 1.0 / t2019 / t785;
    (t5493, t5494, t5497, t5498, t5499, t5502, t5503, t5507)
}
