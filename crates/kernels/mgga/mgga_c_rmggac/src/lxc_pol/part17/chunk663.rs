//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 663/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk663<F: Float>(t1624: F, t236: F, t9188: F, t3351: F, t1627: F, t511: F, t3352: F, t515: F, t8377: F, t2286: F, t7720: F, t495: F, t558: F) -> (F, F, F, F, F, F, F, F) {
    let t9189 = t236 * t1624;
    let t9190 = t9188 * t9189;
    let t9191 = t3351 * t9190;
    let t9193 = t511 * t1627;
    let t9194 = t3352 * t9193;
    let t9195 = t3351 * t9194;
    let t9197 = t515 * t8377;
    let t9198 = t3352 * t9197;
    let t9199 = t3351 * t9198;
    let t9202 = t7720 * t2286;
    let t9205 = t511 * t558 * t495;
    (t9190, t9191, t9194, t9195, t9198, t9199, t9202, t9205)
}
