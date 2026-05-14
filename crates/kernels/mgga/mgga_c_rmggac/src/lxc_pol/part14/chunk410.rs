//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 410/952 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk410<F: Float>(t4161: F, t431: F, t1034: F, t171: F, t433: F, t1045: F, t973: F, t1042: F, t500: F, t998: F, t1003: F, t230: F, t1001: F, t195: F, t1131: F, t388: F) -> (F, F, F, F, F, F, F, F) {
    let t4163 = 0.10254018858216406658e4 * t431 * t4161;
    let t4164 = t1034 * t171;
    let t4165 = t4164 * t433;
    let t4167 = t1045 * t973;
    let t4169 = t1045 * t1042;
    let t4173 = t500 * t998;
    let t4179 = 1.0 / t1003 / t230;
    let t4182 = t195 * t1001;
    let t4186 = t388 * t1131;
    (t4163, t4165, t4167, t4169, t4173, t4179, t4182, t4186)
}
