//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 400/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk400<F: Float>(t189: F, t53: F, t191: F, t60: F, t1346: F, t49: F, t288: F, t325: F, t504: F, t507: F, t837: F) -> (F, F, F, F, F, F) {
    let t3985 = 1.0 / t189 / t53;
    let t3998 = 1.0 / t191 / t60;
    let t4035 = t1346 * t49;
    let t4036 = t4035 * t288;
    let t4041 = t504 * t325;
    let t4044 = t507 * t837;
    (t3985, t3998, t4035, t4036, t4041, t4044)
}
