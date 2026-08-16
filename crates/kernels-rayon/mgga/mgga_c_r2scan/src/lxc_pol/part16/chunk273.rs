//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 273/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk273(t322: f64, t343: f64, t352: f64, t838: f64, t839: f64, t841: f64, t843: f64, t845: f64, t847: f64, t849: f64, t855: f64, t856: f64, t758: f64, t761: f64) -> (f64, f64) {
    let t323 = t322 <= 0.0_f64;
    let t331 = t322 <= 0.25e1_f64;
    let t860 = piecewise5(t323, t838, t331, -0.64e0_f64 * t839 - 0.8704e0_f64 * t841 - 0.4607056813647e1_f64 * t843 + 0.122462410087e2_f64 * t845 - 0.957855118103e1_f64 * t847 + 0.3101306810232e1_f64 * t849 - 0.362942158544e0_f64 * t343 * t839, -0.105e1_f64 * t855 * t856 * t352);
    let t862 = t758 * t761;
    (t860, t862)
}
