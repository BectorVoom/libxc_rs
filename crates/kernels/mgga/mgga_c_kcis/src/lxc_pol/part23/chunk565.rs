//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 565/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk565<F: Float>(t5653: F, t5654: F, t4170: F, t4160: F, t3751: F, t540: F, t1017: F, t86: F) -> (F, F, F, F, F) {
    let t5655 = t5653 * t5654;
    let t5656 = t4170 * t5655;
    let t5657 = t4160 * t5656;
    let t5659 = t3751 * t540;
    let t5661 = t86 * t1017 * t5659;
    (t5655, t5656, t5657, t5659, t5661)
}
