//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2800/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2800(t5555: f64, t9541: f64, t210: f64, t214: f64, t2571: f64, t41200: f64, t46782: f64, t46788: f64, t46790: f64, t46793: f64, t46796: f64, t46802: f64, t46806: f64, t46819: f64, t46828: f64, t46830: f64, t46836: f64, t58090: f64) -> f64 {
    let t59195 = t9541 * t5555;
    let t59197 = -0.2111111111111111111e-1_f64 * t46782 - t41200 + 0.77777777777777777775e-2_f64 * t46788 + 0.11234567901234567901e0_f64 * t46790 + 0.15555555555555555555e0_f64 * t46793 + 0.6333333333333333333e-1_f64 * t46796 + 0.19999999999999999999e-1_f64 * t46802 + 0.55555555555555555553e-3_f64 * t46806 - 0.99999999999999999996e-2_f64 * t46819 - 0.49999999999999999998e-2_f64 * t46828 - 0.46666666666666666664e-1_f64 * t46830 - 0.23333333333333333332e-1_f64 * t46836 + 0.99999999999999999996e-2_f64 * t2571 * t210 * t214 * t58090 - 0.12962962962962962962e-1_f64 * t59195;
    t59197
}
