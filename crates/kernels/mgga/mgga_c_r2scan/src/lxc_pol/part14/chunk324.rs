//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 324/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk324<F: Float>(t322: F, t1126: F, t1127: F, t1129: F, t1131: F, t1133: F, t1135: F, t1137: F, t1142: F, t343: F, t352: F, t855: F, t1106: F, t1118: F, t49: F, t415: F) -> (F, F, F, F) {
    let t323 = t322 <= 0.0;
    let t331 = t322 <= 0.25e1;
    let t1146 = piecewise5(t323, t1126, t331, -0.64e0 * t1127 - 0.8704e0 * t1129 - 0.4607056813647e1 * t1131 + 0.122462410087e2 * t1133 - 0.957855118103e1 * t1135 + 0.3101306810232e1 * t1137 - 0.362942158544e0 * t343 * t1127, -0.105e1 * t855 * t1142 * t352);
    let t1149 = 0.30487649791575028312e-3 * t1106 - t1118;
    let t1212 = 1.0 / t49;
    let t1213 = t415 * t415;
    (t1146, t1149, t1212, t1213)
}
