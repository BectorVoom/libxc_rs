//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 703/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk703(t8822: f64, t8844: f64, t8846: f64, t8872: f64, t8881: f64, t8885: f64, t9040: f64, t9047: f64, t9060: f64, t9062: f64, t9071: f64, t9073: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9440 = 0.2993560425465952141e-1_f64 * t8822;
    let t9492 = 0.1064114997332445985e-4_f64 * t8844;
    let t9493 = 0.1064114997332445985e-4_f64 * t8846;
    let t9501 = 0.8980681276397856423e-1_f64 * t8872;
    let t9600 = 0.2993560425465952141e-1_f64 * t8881;
    let t9601 = 0.8980681276397856423e-1_f64 * t8885;
    let t9603 = 0.19863479950205658386e-4_f64 * t9040;
    let t9605 = 0.1064114997332445985e-4_f64 * t9047;
    let t9611 = 0.23948483403727617128e0_f64 * t9060;
    let t9612 = 0.15965655602485078085e0_f64 * t9062;
    let t9613 = 0.5987120850931904282e-1_f64 * t9071;
    let t9614 = 0.5987120850931904282e-1_f64 * t9073;
    (t9440, t9492, t9493, t9501, t9600, t9601, t9603, t9605, t9611, t9612, t9613, t9614)
}
