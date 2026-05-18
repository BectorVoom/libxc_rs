//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 410/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk410<F: Float>(t930: F, t941: F, t189: F, t53: F, t191: F, t60: F, t356: F, t934: F, t1276: F, t290: F, t1288: F, t68: F) -> (F, F, F, F, F, F) {
    let t3981 = t941 * t930;
    let t3985 = F::new(1.0) / t189 / t53;
    let t3998 = F::new(1.0) / t191 / t60;
    let t4018 = t934 * t356;
    let t4025 = t290 * t1276;
    let t4028 = t68 * t1288;
    (t3981, t3985, t3998, t4018, t4025, t4028)
}
