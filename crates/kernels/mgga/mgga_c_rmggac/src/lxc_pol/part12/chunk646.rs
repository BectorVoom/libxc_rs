//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 646/951 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk646<F: Float>(t1624: F, t236: F, t9188: F, t3351: F, t1627: F, t511: F, t3352: F, t515: F, t8377: F, t2286: F, t7720: F, t495: F, t558: F, t1971: F, t7230: F, t109: F, t4179: F) -> (F, F, F, F, F, F, F, F, F, F) {
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
    let t9206 = t1971 * t9205;
    let t9207 = t7230 * t9206;
    let t9209 = t4179 * t109;
    (t9190, t9191, t9194, t9195, t9198, t9199, t9202, t9206, t9207, t9209)
}
