//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 791/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk791<F: Float>(t4811: F, t5070: F, t142: F, t79: F, t140: F, t1797: F, t1803: F, t5074: F, t5200: F, t227: F, t4596: F) -> (F, F, F, F, F, F) {
    let t10469 = t4811 * t5070;
    let t10471 = t142 * t79;
    let t10473 = t140 * t10471 * t1797;
    let t10474 = t10473 * t1803;
    let t10484 = t5074 * t5200;
    let t10487 = 1.0 / t4596 / t227;
    (t10469, t10471, t10473, t10474, t10484, t10487)
}
