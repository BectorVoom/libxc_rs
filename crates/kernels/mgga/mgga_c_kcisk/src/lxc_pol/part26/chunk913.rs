//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 913/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk913<F: Float>(t1375: F, t25313: F, t1383: F, t1398: F, t8130: F, t960: F, t8133: F, t965: F, t8136: F, t1390: F, t7710: F, t1056: F) -> (F, F, F, F, F, F, F) {
    let t25416 = t1375 * t25313;
    let t25419 = t1383 * t25313;
    let t25422 = t1398 * t25313;
    let t25425 = t960 * t8130;
    let t25427 = t965 * t8133;
    let t25429 = t965 * t8136;
    let t25431 = t1390 * t7710;
    let t25432 = t25431 * t1056;
    (t25416, t25419, t25422, t25425, t25427, t25429, t25432)
}
