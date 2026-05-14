//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 918/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk918<F: Float>(t10835: F, t10843: F, t10854: F, t10864: F, t10867: F, t10902: F, t11817: F, t12192: F, t12193: F, t12534: F, t12536: F, t12539: F, t12541: F, t12544: F, t12548: F, t12552: F) -> (F,) {
    let t12554 = -0.43663693315433241792e-2 * t12534 + 0.43663693315433241792e-2 * t12536 + 0.21831846657716620896e-2 * t12539 + 0.43341108700271342816e-1 * t12541 - 0.13099107994629972538e-1 * t12544 + t10835 + t10843 + 0.47609969197673950972e-2 * t11817 - t10854 + t10864 + t10867 + t12192 + t12193 + 0.2600466522016280569e0 * t12548 - t10902 - 0.21831846657716620896e-2 * t12552;
    (t12554,)
}
