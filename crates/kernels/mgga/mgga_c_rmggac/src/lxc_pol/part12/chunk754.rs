//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 754/951 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk754<F: Float>(t1001: F, t3351: F, t511: F, t558: F, t9210: F, t16043: F, t9184: F, t1652: F, t498: F, t515: F, t7231: F, t29892: F, t3352: F, t2010: F, t2012: F, t5061: F) -> (F, F, F, F, F) {
    let t38717 = t3351 * t9210 * t511 * t558 * t1001;
    let t38719 = t16043 * t9184;
    let t38724 = t3351 * t7231 * t515 * t1652 * t498;
    let t38728 = t3351 * t3352 * t515 * t29892;
    let t38733 = t2010 * t2012 * t5061;
    (t38717, t38719, t38724, t38728, t38733)
}
