//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 647/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk647(t341: f64, t3648: f64, t1020: f64, t1083: f64, t1085: f64, t1087: f64, t1089: f64, t1091: f64, t343: f64, t3650: f64, t3652: f64, t3656: f64, t3660: f64, t3664: f64) -> (f64, f64) {
    let t3668 = t341 * t3648;
    let t3674 = -0.64e0_f64 * t3648 - 0.8704e0_f64 * t3650 - 0.8704e0_f64 * t3652 - 0.9214113627294e1_f64 * t1083 * t1020 - 0.4607056813647e1_f64 * t3656 + 0.367387230261e2_f64 * t1085 * t1020 + 0.122462410087e2_f64 * t3660 - 0.3831420472412e2_f64 * t1087 * t1020 - 0.957855118103e1_f64 * t3664 + 0.1550653405116e2_f64 * t1089 * t1020 + 0.3101306810232e1_f64 * t3668 - 0.2177652951264e1_f64 * t1091 * t1020 - 0.362942158544e0_f64 * t343 * t3648;
    (t3668, t3674)
}
