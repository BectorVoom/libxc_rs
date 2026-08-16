//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 859/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk859(t8815: f64, t9435: f64, t9438: f64, t8822: f64, t9488: f64, t8832: f64, t8837: f64, t8844: f64, t8846: f64, t8852: f64, t8856: f64, t8860: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t42518 = 0.5107751987195740728e-4_f64 * t8815;
    let t42519 = 0.4726e1_f64 * t9435;
    let t42520 = 0.11974241701863808564e0_f64 * t9438;
    let t42521 = 0.5987120850931904282e-1_f64 * t8822;
    let t42527 = 0.39914139006212695214e-1_f64 * t9488;
    let t42528 = 0.638468998399467591e-4_f64 * t8832;
    let t42529 = 0.638468998399467591e-4_f64 * t8837;
    let t42530 = 0.212822999466489197e-4_f64 * t8844;
    let t42531 = 0.212822999466489197e-4_f64 * t8846;
    let t42534 = 0.60975299583150056624e-3_f64 * t8852;
    let t42535 = 0.60975299583150056624e-3_f64 * t8856;
    let t42536 = 0.60975299583150056624e-3_f64 * t8860;
    (t42518, t42519, t42520, t42521, t42527, t42528, t42529, t42530, t42531, t42534, t42535, t42536)
}
