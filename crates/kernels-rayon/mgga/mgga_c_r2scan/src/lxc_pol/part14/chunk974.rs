//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 974/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk974(t10758: f64, t10753: f64, t10762: f64, t10765: f64, t10770: f64, t10774: f64, t10778: f64, t10782: f64, t10786: f64, t10790: f64, t10794: f64, t10797: f64, t10801: f64, t10803: f64, t10806: f64) -> (f64, f64) {
    let t11399 = 0.11902492299418487743e0_f64 * t10758;
    let t11413 = 0.17336443480108537126e0_f64 * t10753 + t11399 - 0.87327386630866483588e-2_f64 * t10762 - 0.26198215989259945076e-1_f64 * t10765 + 0.95219938395347901946e-2_f64 * t10770 + 0.5200933044032561138e0_f64 * t10774 + 0.17336443480108537126e0_f64 * t10778 + 0.21951497276451705328e0_f64 * t10782 - 0.87327386630866483588e-2_f64 * t10786 - 0.52396431978519890152e-1_f64 * t10790 + 0.43663693315433241794e-2_f64 * t10794 + 0.26198215989259945076e-1_f64 * t10797 + 0.13099107994629972538e-1_f64 * t10801 + 0.21951497276451705328e-1_f64 * t10803 - 0.54878743191129263322e-1_f64 * t10806;
    (t11399, t11413)
}
