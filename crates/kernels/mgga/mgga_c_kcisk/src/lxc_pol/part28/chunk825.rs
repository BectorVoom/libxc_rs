//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 825/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk825<F: Float>(t142: F, t79: F, t140: F, t1797: F, t1803: F, t227: F, t4596: F) -> (F, F, F, F) {
    let t10471 = t142 * t79;
    let t10473 = t140 * t10471 * t1797;
    let t10474 = t10473 * t1803;
    let t10487 = 1.0 / t4596 / t227;
    (t10471, t10473, t10474, t10487)
}
