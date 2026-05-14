//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1074/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1074<F: Float>(t3179: F, t3344: F, t10879: F, t12503: F, t43528: F, t43531: F, t43533: F, t43536: F, t43539: F, t43541: F, t43544: F, t43547: F, t43549: F, t43551: F, t261: F, t3304: F, t9311: F) -> (F, F) {
    let t43553 = t3179 * t3344;
    let t43555 = t10879 * t12503;
    let t43557 = -0.43663693315433241792e-2 * t43528 - 0.43663693315433241792e-2 * t43531 + 0.43663693315433241792e-2 * t43533 - 0.21831846657716620896e-2 * t43536 - 0.65495539973149862688e-2 * t43539 - 0.13099107994629972538e-1 * t43541 - 0.13099107994629972538e-1 * t43544 - 0.13099107994629972538e-1 * t43547 - 0.43341108700271342816e-1 * t43549 - 0.43663693315433241792e-2 * t43551 - 0.23804984598836975486e-2 * t43553 - 0.13002332610081402845e0 * t43555;
    let t43559 = t3304 * t261 * t9311;
    (t43557, t43559)
}
