//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1240/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1240(t38622: f64, t40107: f64, t40109: f64, t43559: f64, t43561: f64, t43565: f64, t43569: f64, t43572: f64, t43575: f64, t43577: f64, t43579: f64, t43581: f64) -> f64 {
    let t44452 = -0.2600466522016280569e0_f64 * t43559 - 0.5200933044032561138e0_f64 * t43561 - t38622 - 0.2600466522016280569e0_f64 * t43565 + 0.11708928647259339622e0_f64 * t40107 - 0.54878743191129263322e-1_f64 * t43569 - 0.52009330440325611378e0_f64 * t43572 - 0.52009330440325611378e0_f64 * t43575 + 0.5200933044032561138e0_f64 * t43577 + 0.20803732176130244552e1_f64 * t43579 + 0.16951189180550569635e1_f64 * t40109 - 0.97574405393827830187e-2_f64 * t43581;
    t44452
}
