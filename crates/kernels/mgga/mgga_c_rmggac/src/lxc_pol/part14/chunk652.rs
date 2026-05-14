//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 652/952 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk652<F: Float>(t8692: F, t8698: F, t8822: F, t8844: F, t8846: F, t8872: F, t8881: F, t8885: F, t9040: F, t9047: F, t9060: F, t9062: F, t9071: F, t9073: F, t9091: F, t9124: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t9419 = 0.19863479950205658386e-4 * t8692;
    let t9422 = 0.19863479950205658386e-4 * t8698;
    let t9440 = 0.2993560425465952141e-1 * t8822;
    let t9492 = 0.1064114997332445985e-4 * t8844;
    let t9493 = 0.1064114997332445985e-4 * t8846;
    let t9501 = 0.8980681276397856423e-1 * t8872;
    let t9600 = 0.2993560425465952141e-1 * t8881;
    let t9601 = 0.8980681276397856423e-1 * t8885;
    let t9603 = 0.19863479950205658386e-4 * t9040;
    let t9605 = 0.1064114997332445985e-4 * t9047;
    let t9611 = 0.23948483403727617128e0 * t9060;
    let t9612 = 0.15965655602485078085e0 * t9062;
    let t9613 = 0.5987120850931904282e-1 * t9071;
    let t9614 = 0.5987120850931904282e-1 * t9073;
    let t9619 = 0.19863479950205658386e-4 * t9091;
    let t9636 = 0.1064114997332445985e-4 * t9124;
    (t9419, t9422, t9440, t9492, t9493, t9501, t9600, t9601, t9603, t9605, t9611, t9612, t9613, t9614, t9619, t9636)
}
