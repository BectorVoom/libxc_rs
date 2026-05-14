//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1120/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1120<F: Float>(t2692: F, t3187: F, t10336: F, t1147: F, t9404: F, t15461: F, t1121: F, t397: F, t1123: F, t1128: F, t3376: F) -> (F, F, F, F, F, F, F) {
    let t32581 = t2692 * t3187;
    let t32582 = t10336 * t32581;
    let t32583 = 6.0 * t32582;
    let t32584 = t9404 * t1147;
    let t32588 = t15461 * t2692;
    let t32589 = t397 * t1121;
    let t32592 = t32589 * t3376 * t1123 * t1128;
    (t32581, t32582, t32583, t32584, t32588, t32589, t32592)
}
