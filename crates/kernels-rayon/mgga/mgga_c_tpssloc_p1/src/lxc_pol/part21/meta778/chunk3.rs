//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2693/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2693(t19783: f64, t54670: f64, t16081: f64, t19787: f64, t5187: f64, t5308: f64, t16018: f64, t16101: f64, t19781: f64, t221: f64, t3719: f64, t46838: f64, t5195: f64, t5196: f64, t54673: f64, t54676: f64, t54690: f64, t54698: f64, t54701: f64, t54705: f64, t54711: f64, t54721: f64, t54725: f64) -> (f64, f64) {
    let t56548 = t54670 * t19783;
    let t56550 = t16081 * t19787;
    let t56560 = t5308 * t5187;
    let t56568 = -0.23333333333333333332e-1_f64 * t54673 + 0.6333333333333333333e-1_f64 * t54676 - 0.99999999999999999996e-2_f64 * t54690 - 0.49999999999999999998e-2_f64 * t54698 + 0.15555555555555555555e0_f64 * t54701 + 0.93333333333333333328e-1_f64 * t56548 - 0.46666666666666666664e-1_f64 * t56550 - 0.19999999999999999999e-1_f64 * t16101 * t221 * t19781 * t3719 + 0.99999999999999999996e-2_f64 * t5195 * t221 * t5196 * t16018 - 0.79999999999999999996e-1_f64 * t16101 * t46838 * t56560 + 0.16666666666666666666e-2_f64 * t54705 - 0.46666666666666666664e-1_f64 * t54711 + 0.19999999999999999999e-1_f64 * t54721 + 0.55555555555555555553e-3_f64 * t54725;
    (t56560, t56568)
}
