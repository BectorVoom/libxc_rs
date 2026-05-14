//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 789/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk789<F: Float>(t118: F, t1184: F, t1986: F, t44586: F, t35191: F, t1971: F, t236: F, t495: F, t6182: F, t7453: F, t498: F, t6108: F, t7231: F, t7365: F, t321: F, t3352: F) -> (F, F, F, F) {
    let t44589 = t1986 * t118 * t44586 * t1184;
    let t44590 = t35191 * t44589;
    let t44595 = t7453 * t1971 * t236 * t6182 * t495;
    let t44600 = t7365 * t7231 * t236 * t6108 * t498;
    let t44605 = t7365 * t3352 * t236 * t6108 * t321;
    (t44590, t44595, t44600, t44605)
}
