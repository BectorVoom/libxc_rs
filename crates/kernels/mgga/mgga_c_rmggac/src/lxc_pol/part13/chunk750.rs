//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 750/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk750<F: Float>(t29892: F, t3351: F, t3352: F, t515: F, t2010: F, t2012: F, t5061: F, t4601: F, t8551: F, t2060: F, t31125: F, t903: F, t321: F, t8700: F, t262: F, t7198: F) -> (F, F, F, F, F, F, F) {
    let t38728 = t3351 * t3352 * t515 * t29892;
    let t38733 = t2010 * t2012 * t5061;
    let t38739 = t4601 * t8551;
    let t38742 = t903 * t2060 * t31125;
    let t38745 = t8700 * t321;
    let t38746 = t262 * t38745;
    let t38747 = t7198 * t38746;
    (t38728, t38733, t38739, t38742, t38745, t38746, t38747)
}
