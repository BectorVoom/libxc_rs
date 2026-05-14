//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 893/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk893<F: Float>(t10758: F, t10753: F, t10762: F, t10765: F, t10770: F, t10774: F, t10778: F, t10782: F, t10786: F, t10790: F, t10794: F, t10797: F, t10801: F, t10803: F, t10806: F, t10818: F) -> (F, F, F) {
    let t11399 = 0.11902492299418487743e0 * t10758;
    let t11413 = 0.17336443480108537126e0 * t10753 + t11399 - 0.87327386630866483588e-2 * t10762 - 0.26198215989259945076e-1 * t10765 + 0.95219938395347901946e-2 * t10770 + 0.5200933044032561138e0 * t10774 + 0.17336443480108537126e0 * t10778 + 0.21951497276451705328e0 * t10782 - 0.87327386630866483588e-2 * t10786 - 0.52396431978519890152e-1 * t10790 + 0.43663693315433241794e-2 * t10794 + 0.26198215989259945076e-1 * t10797 + 0.13099107994629972538e-1 * t10801 + 0.21951497276451705328e-1 * t10803 - 0.54878743191129263322e-1 * t10806;
    let t11417 = 0.58544643236296698113e-1 * t10818;
    (t11399, t11413, t11417)
}
