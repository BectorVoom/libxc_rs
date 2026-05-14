//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 613/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk613<F: Float>(t10464: F, t1801: F, t1800: F, t10461: F, t4811: F, t5070: F, t142: F, t79: F, t140: F, t1797: F, t1803: F, t4581: F, t5199: F, t1799: F, t1894: F, t4972: F) -> (F, F, F, F, F, F) {
    let t10465 = t1801 * t10464;
    let t10466 = t1800 * t10465;
    let t10467 = t10461 * t10466;
    let t10469 = t4811 * t5070;
    let t10471 = t142 * t79;
    let t10473 = t140 * t10471 * t1797;
    let t10474 = t10473 * t1803;
    let t10476 = t4581 * t5199;
    let t10477 = t1799 * t10476;
    let t10479 = t1894 * t4972;
    (t10467, t10469, t10471, t10474, t10477, t10479)
}
