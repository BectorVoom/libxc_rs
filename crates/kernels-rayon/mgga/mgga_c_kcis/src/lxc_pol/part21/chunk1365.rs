//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1365/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1365(t26966: f64, t28214: f64, t15198: f64, t2197: f64, t27014: f64, t27020: f64, t28146: f64, t28190: f64, t7779: f64, t8091: f64, t92613: f64, t92991: f64, t92993: f64, t92997: f64, t96042: f64, t96045: f64, t96048: f64, t96052: f64) -> f64 {
    let t97212 = t26966 * t28214;
    let t97215 = 0.77382407407407407407e-3_f64 * t92991 + 0.77382407407407407406e-3_f64 * t92993 - 0.51588271604938271604e-3_f64 * t92997 + 0.92858888888888888886e-2_f64 * t96042 + 0.34752604166666666667e-3_f64 * t28190 * t27020 + 0.18534722222222222222e-2_f64 * t15198 * t7779 * t2197 - 0.92858888888888888888e-2_f64 * t96045 - 0.46336805555555555556e-3_f64 * t27014 * t28146 - 0.11326774691358024691e-2_f64 * t92613 * t8091 - 0.15476481481481481481e-2_f64 * t96048 + 0.20594135802469135802e-3_f64 * t97212 + 0.23214722222222222222e-2_f64 * t96052;
    t97215
}
