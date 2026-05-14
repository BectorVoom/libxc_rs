//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1079/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1079<F: Float>(t43528: F, t43531: F, t43533: F, t43536: F, t43539: F, t43541: F, t43544: F, t43547: F, t43549: F, t43551: F, t43553: F, t43555: F, t38622: F, t40107: F, t40109: F, t43559: F, t43561: F, t43565: F, t43569: F, t43572: F, t43575: F, t43577: F, t43579: F, t43581: F) -> (F, F) {
    let t44440 = -0.87327386630866483588e-2 * t43528 - 0.87327386630866483588e-2 * t43531 + 0.87327386630866483588e-2 * t43533 - 0.43663693315433241794e-2 * t43536 - 0.13099107994629972538e-1 * t43539 - 0.26198215989259945076e-1 * t43541 - 0.26198215989259945076e-1 * t43544 - 0.26198215989259945076e-1 * t43547 - 0.86682217400542685632e-1 * t43549 - 0.87327386630866483588e-2 * t43551 - 0.47609969197673950973e-2 * t43553 - 0.2600466522016280569e0 * t43555;
    let t44452 = -0.2600466522016280569e0 * t43559 - 0.5200933044032561138e0 * t43561 - t38622 - 0.2600466522016280569e0 * t43565 + 0.11708928647259339622e0 * t40107 - 0.54878743191129263322e-1 * t43569 - 0.52009330440325611378e0 * t43572 - 0.52009330440325611378e0 * t43575 + 0.5200933044032561138e0 * t43577 + 0.20803732176130244552e1 * t43579 + 0.16951189180550569635e1 * t40109 - 0.97574405393827830187e-2 * t43581;
    (t44440, t44452)
}
