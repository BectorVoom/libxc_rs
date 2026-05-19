//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 703/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk703<F: Float>(t8822: F, t8844: F, t8846: F, t8872: F, t8881: F, t8885: F, t9040: F, t9047: F, t9060: F, t9062: F, t9071: F, t9073: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t9440 = F::cast_from(0.2993560425465952141e-1_f64) * t8822;
    let t9492 = F::cast_from(0.1064114997332445985e-4_f64) * t8844;
    let t9493 = F::cast_from(0.1064114997332445985e-4_f64) * t8846;
    let t9501 = F::cast_from(0.8980681276397856423e-1_f64) * t8872;
    let t9600 = F::cast_from(0.2993560425465952141e-1_f64) * t8881;
    let t9601 = F::cast_from(0.8980681276397856423e-1_f64) * t8885;
    let t9603 = F::cast_from(0.19863479950205658386e-4_f64) * t9040;
    let t9605 = F::cast_from(0.1064114997332445985e-4_f64) * t9047;
    let t9611 = F::cast_from(0.23948483403727617128e0_f64) * t9060;
    let t9612 = F::cast_from(0.15965655602485078085e0_f64) * t9062;
    let t9613 = F::cast_from(0.5987120850931904282e-1_f64) * t9071;
    let t9614 = F::cast_from(0.5987120850931904282e-1_f64) * t9073;
    (t9440, t9492, t9493, t9501, t9600, t9601, t9603, t9605, t9611, t9612, t9613, t9614)
}
