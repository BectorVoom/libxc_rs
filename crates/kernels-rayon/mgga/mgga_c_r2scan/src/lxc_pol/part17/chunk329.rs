//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 329/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk329(t322: f64, t1126: f64, t1127: f64, t1129: f64, t1131: f64, t1133: f64, t1135: f64, t1137: f64, t1142: f64, t343: f64, t352: f64, t855: f64, t1106: f64, t1118: f64) -> (f64, f64) {
    let t323 = t322 <= 0.0_f64;
    let t331 = t322 <= 0.25e1_f64;
    let t1146 = piecewise5(t323, t1126, t331, -0.64e0_f64 * t1127 - 0.8704e0_f64 * t1129 - 0.4607056813647e1_f64 * t1131 + 0.122462410087e2_f64 * t1133 - 0.957855118103e1_f64 * t1135 + 0.3101306810232e1_f64 * t1137 - 0.362942158544e0_f64 * t343 * t1127, -0.105e1_f64 * t855 * t1142 * t352);
    let t1149 = 0.30487649791575028312e-3_f64 * t1106 - t1118;
    (t1146, t1149)
}
