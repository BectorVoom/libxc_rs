//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 670/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk670<F: Float>(t8881: F, t8885: F, t9047: F, t9071: F, t9073: F, t7910: F, t9124: F, t9126: F, t9129: F, t9148: F, t9223: F, t9225: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t9600 = F::cast_from(0.2993560425465952141e-1_f64) * t8881;
    let t9601 = F::cast_from(0.8980681276397856423e-1_f64) * t8885;
    let t9605 = F::cast_from(0.1064114997332445985e-4_f64) * t9047;
    let t9613 = F::cast_from(0.5987120850931904282e-1_f64) * t9071;
    let t9614 = F::cast_from(0.5987120850931904282e-1_f64) * t9073;
    let t9631 = F::cast_from(0.59590439850616975158e-4_f64) * t7910;
    let t9636 = F::cast_from(0.1064114997332445985e-4_f64) * t9124;
    let t9646 = F::cast_from(0.2993560425465952141e-1_f64) * t9126;
    let t9647 = F::cast_from(0.5987120850931904282e-1_f64) * t9129;
    let t9653 = F::cast_from(0.1064114997332445985e-4_f64) * t9148;
    let t9670 = F::cast_from(0.1064114997332445985e-4_f64) * t9223;
    let t9671 = F::cast_from(0.8980681276397856423e-1_f64) * t9225;
    (t9600, t9601, t9605, t9613, t9614, t9631, t9636, t9646, t9647, t9653, t9670, t9671)
}
